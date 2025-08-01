use std::{
    io::{Read, Seek, Write},
    ops::RangeInclusive,
};

use helix_core::diagnostic::Severity;
use helix_stdx::path;
use helix_view::doc;

use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_write_quit_fail() -> anyhow::Result<()> {
    let file = helpers::new_readonly_tempfile()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ihello<esc>:wq<ret>"),
        Some(&|app| {
            let mut docs: Vec<_> = app.editor.documents().collect();
            assert_eq!(1, docs.len());

            let doc = docs.pop().unwrap();
            assert_eq!(Some(&path::normalize(file.path())), doc.path());
            assert_eq!(&Severity::Error, app.editor.get_status().unwrap().1);
        }),
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_buffer_close_concurrent() -> anyhow::Result<()> {
    test_key_sequences(
        &mut helpers::AppBuilder::new().build()?,
        vec![
            (
                None,
                Some(&|app| {
                    assert_eq!(1, app.editor.documents().count());
                    assert!(!app.editor.is_err());
                }),
            ),
            (
                Some("ihello<esc>:new<ret>"),
                Some(&|app| {
                    assert_eq!(2, app.editor.documents().count());
                    assert!(!app.editor.is_err());
                }),
            ),
            (
                Some(":buffer<minus>close<ret>"),
                Some(&|app| {
                    assert_eq!(1, app.editor.documents().count());
                    assert!(!app.editor.is_err());
                }),
            ),
        ],
        false,
    )
    .await?;

    // verify if writes are queued up, it finishes them before closing the buffer
    let mut file = tempfile::NamedTempFile::new()?;
    let mut command = String::new();
    const RANGE: RangeInclusive<i32> = 1..=1000;

    for i in RANGE {
        let cmd = format!("%c{}<esc>:w!<ret>", i);
        command.push_str(&cmd);
    }

    command.push_str(":buffer<minus>close<ret>");

    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some(&command),
        Some(&|app| {
            assert!(!app.editor.is_err(), "error: {:?}", app.editor.get_status());

            let doc = app.editor.document_by_path(file.path());
            assert!(doc.is_none(), "found doc: {:?}", doc);
        }),
        false,
    )
    .await?;

    helpers::assert_file_has_content(
        &mut file,
        &LineFeedHandling::Native.apply(&RANGE.end().to_string()),
    )?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ithe gostak distims the doshes<ret><esc>:w<ret>"),
        None,
        false,
    )
    .await?;

    reload_file(&mut file).unwrap();
    let mut file_content = String::new();
    file.as_file_mut().read_to_string(&mut file_content)?;

    assert_eq!(
        LineFeedHandling::Native.apply("the gostak distims the doshes"),
        file_content
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_overwrite_protection() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    helpers::run_event_loop_until_idle(&mut app).await;

    file.as_file_mut()
        .write_all("extremely important content".as_bytes())?;

    file.as_file_mut().flush()?;
    file.as_file_mut().sync_all()?;

    test_key_sequence(&mut app, Some(":x<ret>"), None, false).await?;

    reload_file(&mut file).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    assert_eq!("extremely important content", file_content);

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_quit() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ithe gostak distims the doshes<ret><esc>:wq<ret>"),
        None,
        true,
    )
    .await?;

    reload_file(&mut file).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    assert_eq!(
        LineFeedHandling::Native.apply("the gostak distims the doshes"),
        file_content
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_concurrent() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut command = String::new();
    const RANGE: RangeInclusive<i32> = 1..=1000;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    for i in RANGE {
        let cmd = format!("%c{}<esc>:w!<ret>", i);
        command.push_str(&cmd);
    }

    test_key_sequence(&mut app, Some(&command), None, false).await?;

    reload_file(&mut file).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    assert_eq!(
        LineFeedHandling::Native.apply(&RANGE.end().to_string()),
        file_content
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_fail_mod_flag() -> anyhow::Result<()> {
    let file = helpers::new_readonly_tempfile()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    test_key_sequences(
        &mut app,
        vec![
            (
                None,
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(!doc.is_modified());
                }),
            ),
            (
                Some("ihello<esc>"),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(doc.is_modified());
                }),
            ),
            (
                Some(":w<ret>"),
                Some(&|app| {
                    assert_eq!(&Severity::Error, app.editor.get_status().unwrap().1);

                    let doc = doc!(app.editor);
                    assert!(doc.is_modified());
                }),
            ),
        ],
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_scratch_to_new_path() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;

    test_key_sequence(
        &mut AppBuilder::new().build()?,
        Some(format!("ihello<esc>:w {}<ret>", file.path().to_string_lossy()).as_ref()),
        Some(&|app| {
            assert!(!app.editor.is_err());

            let mut docs: Vec<_> = app.editor.documents().collect();
            assert_eq!(1, docs.len());

            let doc = docs.pop().unwrap();
            assert_eq!(Some(&path::normalize(file.path())), doc.path());
        }),
        false,
    )
    .await?;

    helpers::assert_file_has_content(&mut file, &LineFeedHandling::Native.apply("hello"))?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_scratch_no_path_fails() -> anyhow::Result<()> {
    helpers::test_key_sequence_with_input_text(
        None,
        ("#[\n|]#", "ihello<esc>:w<ret>", "hello#[\n|]#"),
        &|app| {
            assert!(app.editor.is_err());

            let mut docs: Vec<_> = app.editor.documents().collect();
            assert_eq!(1, docs.len());

            let doc = docs.pop().unwrap();
            assert_eq!(None, doc.path());
        },
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_auto_format_fails_still_writes() -> anyhow::Result<()> {
    let mut file = tempfile::Builder::new().suffix(".rs").tempfile()?;

    let lang_conf = indoc! {r#"
            [[language]]
            name = "rust"
            formatter = { command = "bash", args = [ "-c", "exit 1" ] }
        "#};

    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .with_input_text("#[l|]#et foo = 0;\n")
        .with_lang_loader(helpers::test_syntax_loader(Some(lang_conf.into())))
        .build()?;

    test_key_sequences(&mut app, vec![(Some(":w<ret>"), None)], false).await?;

    // file still saves
    helpers::assert_file_has_content(&mut file, "let foo = 0;\n")?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_new_path() -> anyhow::Result<()> {
    let mut file1 = tempfile::NamedTempFile::new().unwrap();
    let mut file2 = tempfile::NamedTempFile::new().unwrap();
    let mut app = helpers::AppBuilder::new()
        .with_file(file1.path(), None)
        .build()?;

    test_key_sequences(
        &mut app,
        vec![
            (
                Some("ii can eat glass, it will not hurt me<ret><esc>:w<ret>"),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(!app.editor.is_err());
                    assert_eq!(&path::normalize(file1.path()), doc.path().unwrap());
                }),
            ),
            (
                Some(&format!(":w {}<ret>", file2.path().to_string_lossy())),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(!app.editor.is_err());
                    assert_eq!(&path::normalize(file2.path()), doc.path().unwrap());
                    assert!(app.editor.document_by_path(file1.path()).is_none());
                }),
            ),
        ],
        false,
    )
    .await?;

    helpers::assert_file_has_content(
        &mut file1,
        &LineFeedHandling::Native.apply("i can eat glass, it will not hurt me\n"),
    )?;

    helpers::assert_file_has_content(
        &mut file2,
        &LineFeedHandling::Native.apply("i can eat glass, it will not hurt me\n"),
    )?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_fail_new_path() -> anyhow::Result<()> {
    let file = helpers::new_readonly_tempfile()?;

    test_key_sequences(
        &mut AppBuilder::new().build()?,
        vec![
            (
                None,
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert_ne!(
                        Some(&Severity::Error),
                        app.editor.get_status().map(|status| status.1)
                    );
                    assert_eq!(None, doc.path());
                }),
            ),
            (
                Some(&format!(":w {}<ret>", file.path().to_string_lossy())),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert_eq!(
                        Some(&Severity::Error),
                        app.editor.get_status().map(|status| status.1)
                    );
                    assert_eq!(None, doc.path());
                }),
            ),
        ],
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_utf_bom_file() -> anyhow::Result<()> {
    // "ABC" with utf8 bom
    const UTF8_FILE: [u8; 6] = [0xef, 0xbb, 0xbf, b'A', b'B', b'C'];

    // "ABC" in UTF16 with bom
    const UTF16LE_FILE: [u8; 8] = [0xff, 0xfe, b'A', 0x00, b'B', 0x00, b'C', 0x00];
    const UTF16BE_FILE: [u8; 8] = [0xfe, 0xff, 0x00, b'A', 0x00, b'B', 0x00, b'C'];

    edit_file_with_content(&UTF8_FILE).await?;
    edit_file_with_content(&UTF16LE_FILE).await?;
    edit_file_with_content(&UTF16BE_FILE).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_trim_trailing_whitespace() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_config(Config {
            editor: helix_view::editor::Config {
                trim_trailing_whitespace: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_file(file.path(), None)
        .with_input_text(LineFeedHandling::Native.apply("#[f|]#oo      \n\n \nbar      "))
        .build()?;

    test_key_sequence(&mut app, Some(":w<ret>"), None, false).await?;

    helpers::assert_file_has_content(&mut file, &LineFeedHandling::Native.apply("foo\n\n\nbar"))?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_trim_final_newlines() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_config(Config {
            editor: helix_view::editor::Config {
                trim_final_newlines: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_file(file.path(), None)
        .with_input_text(LineFeedHandling::Native.apply("#[f|]#oo\n \n\n\n"))
        .build()?;

    test_key_sequence(&mut app, Some(":w<ret>"), None, false).await?;

    helpers::assert_file_has_content(&mut file, &LineFeedHandling::Native.apply("foo\n \n"))?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_insert_final_newline_added_if_missing() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .with_input_text("#[h|]#ave you tried chamomile tea?")
        .build()?;

    test_key_sequence(&mut app, Some(":w<ret>"), None, false).await?;

    helpers::assert_file_has_content(
        &mut file,
        &LineFeedHandling::Native.apply("have you tried chamomile tea?\n"),
    )?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_insert_final_newline_unchanged_if_empty() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .with_input_text("#[|]#")
        .build()?;

    test_key_sequence(&mut app, Some(":w<ret>"), None, false).await?;

    helpers::assert_file_has_content(&mut file, "")?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_insert_final_newline_unchanged_if_not_missing() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .with_input_text(LineFeedHandling::Native.apply("#[t|]#en minutes, please\n"))
        .build()?;

    test_key_sequence(&mut app, Some(":w<ret>"), None, false).await?;

    helpers::assert_file_has_content(
        &mut file,
        &LineFeedHandling::Native.apply("ten minutes, please\n"),
    )?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_insert_final_newline_unchanged_if_missing_and_false() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_config(Config {
            editor: helix_view::editor::Config {
                insert_final_newline: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_file(file.path(), None)
        .with_input_text("#[t|]#he quiet rain continued through the night")
        .build()?;

    test_key_sequence(&mut app, Some(":w<ret>"), None, false).await?;

    reload_file(&mut file).unwrap();
    helpers::assert_file_has_content(&mut file, "the quiet rain continued through the night")?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_all_insert_final_newline_add_if_missing_and_modified() -> anyhow::Result<()> {
    let mut file1 = tempfile::NamedTempFile::new()?;
    let mut file2 = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file1.path(), None)
        .with_input_text("#[w|]#e don't serve time travelers here")
        .build()?;

    test_key_sequence(
        &mut app,
        Some(&format!(
            ":o {}<ret>ia time traveler walks into a bar<esc>:wa<ret>",
            file2.path().to_string_lossy()
        )),
        None,
        false,
    )
    .await?;

    helpers::assert_file_has_content(
        &mut file1,
        &LineFeedHandling::Native.apply("we don't serve time travelers here\n"),
    )?;

    helpers::assert_file_has_content(
        &mut file2,
        &LineFeedHandling::Native.apply("a time traveler walks into a bar\n"),
    )?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_write_all_insert_final_newline_do_not_add_if_unmodified() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .build()?;

    file.write_all(b"i lost on Jeopardy!")?;
    file.rewind()?;

    test_key_sequence(&mut app, Some(":wa<ret>"), None, false).await?;

    helpers::assert_file_has_content(&mut file, "i lost on Jeopardy!")?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_symlink_write() -> anyhow::Result<()> {
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(not(unix))]
    use std::os::windows::fs::symlink_file as symlink;

    let dir = tempfile::tempdir()?;

    let mut file = tempfile::NamedTempFile::new_in(&dir)?;
    let symlink_path = dir.path().join("linked");
    symlink(file.path(), &symlink_path)?;

    let mut app = helpers::AppBuilder::new()
        .with_file(&symlink_path, None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ithe gostak distims the doshes<ret><esc>:w<ret>"),
        None,
        false,
    )
    .await?;

    reload_file(&mut file).unwrap();
    let mut file_content = String::new();
    file.as_file_mut().read_to_string(&mut file_content)?;

    assert_eq!(
        LineFeedHandling::Native.apply("the gostak distims the doshes"),
        file_content
    );
    assert!(symlink_path.is_symlink());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_symlink_write_fail() -> anyhow::Result<()> {
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(not(unix))]
    use std::os::windows::fs::symlink_file as symlink;

    let dir = tempfile::tempdir()?;

    let file = helpers::new_readonly_tempfile_in_dir(&dir)?;
    let symlink_path = dir.path().join("linked");
    symlink(file.path(), &symlink_path)?;

    let mut app = helpers::AppBuilder::new()
        .with_file(&symlink_path, None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ihello<esc>:wq<ret>"),
        Some(&|app| {
            let mut docs: Vec<_> = app.editor.documents().collect();
            assert_eq!(1, docs.len());

            let doc = docs.pop().unwrap();
            assert_eq!(Some(&path::normalize(&symlink_path)), doc.path());
            assert_eq!(&Severity::Error, app.editor.get_status().unwrap().1);
        }),
        false,
    )
    .await?;

    assert!(symlink_path.is_symlink());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_symlink_write_relative() -> anyhow::Result<()> {
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    #[cfg(not(unix))]
    use std::os::windows::fs::symlink_file as symlink;

    // tempdir
    // |- - b
    // |  |- file
    // |- linked (symlink to file)
    let dir = tempfile::tempdir()?;
    let inner_dir = dir.path().join("b");
    std::fs::create_dir(&inner_dir)?;

    let mut file = tempfile::NamedTempFile::new_in(&inner_dir)?;
    let symlink_path = dir.path().join("linked");
    let relative_path = std::path::PathBuf::from("b").join(file.path().file_name().unwrap());
    symlink(relative_path, &symlink_path)?;

    let mut app = helpers::AppBuilder::new()
        .with_file(&symlink_path, None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ithe gostak distims the doshes<ret><esc>:w<ret>"),
        None,
        false,
    )
    .await?;

    reload_file(&mut file).unwrap();
    let mut file_content = String::new();
    file.as_file_mut().read_to_string(&mut file_content)?;

    assert_eq!(
        LineFeedHandling::Native.apply("the gostak distims the doshes"),
        file_content
    );
    assert!(symlink_path.is_symlink());

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
#[cfg(not(target_os = "android"))]
async fn test_hardlink_write() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;

    let mut file = tempfile::NamedTempFile::new_in(&dir)?;
    let hardlink_path = dir.path().join("linked");
    std::fs::hard_link(file.path(), &hardlink_path)?;

    let mut app = helpers::AppBuilder::new()
        .with_file(&hardlink_path, None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some("ithe gostak distims the doshes<ret><esc>:w<ret>"),
        None,
        false,
    )
    .await?;

    reload_file(&mut file).unwrap();
    let mut file_content = String::new();
    file.as_file_mut().read_to_string(&mut file_content)?;

    assert_eq!(
        LineFeedHandling::Native.apply("the gostak distims the doshes"),
        file_content
    );
    assert!(helix_stdx::faccess::hardlink_count(&hardlink_path)? > 1);
    assert!(same_file::is_same_file(file.path(), &hardlink_path)?);

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_reload_no_force() -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file.path(), None)
        .with_input_text("hello#[ |]#")
        .build()?;

    test_key_sequences(
        &mut app,
        vec![
            (Some("athere<esc>"), None),
            (
                Some(":reload<ret>"),
                Some(&|app| {
                    assert!(app.editor.is_err());

                    let doc = app.editor.documents().next().unwrap();
                    assert!(doc.is_modified());
                    assert_eq!(doc.text(), &LineFeedHandling::Native.apply("hello there"));
                }),
            ),
        ],
        false,
    )
    .await?;

    helpers::assert_file_has_content(&mut file, "")?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_reload_all_no_force() -> anyhow::Result<()> {
    let file1 = tempfile::NamedTempFile::new()?;
    let mut file2 = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file1.path(), None)
        .with_file(file2.path(), None)
        .with_input_text("#[c|]#hange1")
        .build()?;

    file2.as_file_mut().write_all(b"change2")?;

    test_key_sequence(
        &mut app,
        Some(":reload-all<ret>"),
        Some(&|app| {
            assert!(app.editor.is_err());

            let (mut doc1_visited, mut doc2_visited) = (false, false);
            for doc in app.editor.documents() {
                if doc.path().unwrap() == file1.path() {
                    assert!(doc.is_modified());
                    assert_eq!(doc.text(), "change1");
                    doc1_visited = true;
                } else if doc.path().unwrap() == file2.path() {
                    assert!(!doc.is_modified());
                    assert_eq!(doc.text(), "change2");
                    doc2_visited = true;
                }
            }
            assert!(app.editor.documents().count() == 2 && doc1_visited && doc2_visited);
        }),
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_reload_all_force() -> anyhow::Result<()> {
    let file1 = tempfile::NamedTempFile::new()?;
    let mut file2 = tempfile::NamedTempFile::new()?;
    let mut app = helpers::AppBuilder::new()
        .with_file(file1.path(), None)
        .with_file(file2.path(), None)
        .with_input_text("#[c|]#hange1")
        .build()?;

    file2.as_file_mut().write_all(b"change2")?;

    test_key_sequence(
        &mut app,
        Some(":reload-all!<ret>"),
        Some(&|app| {
            assert!(!app.editor.is_err());

            let (mut doc1_visited, mut doc2_visited) = (false, false);
            for doc in app.editor.documents() {
                if doc.path().unwrap() == file1.path() {
                    assert!(!doc.is_modified());
                    assert_eq!(doc.text(), "");
                    doc1_visited = true;
                } else if doc.path().unwrap() == file2.path() {
                    assert!(!doc.is_modified());
                    assert_eq!(doc.text(), "change2");
                    doc2_visited = true;
                }
            }
            assert!(app.editor.documents().count() == 2 && doc1_visited && doc2_visited);
        }),
        false,
    )
    .await?;

    Ok(())
}

async fn edit_file_with_content(file_content: &[u8]) -> anyhow::Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;

    file.as_file_mut().write_all(&file_content)?;

    helpers::test_key_sequence(
        &mut helpers::AppBuilder::new()
            .with_config(Config {
                editor: helix_view::editor::Config {
                    insert_final_newline: false,
                    ..Default::default()
                },
                ..Default::default()
            })
            .build()?,
        Some(&format!(":o {}<ret>:x<ret>", file.path().to_string_lossy())),
        None,
        true,
    )
    .await?;

    reload_file(&mut file).unwrap();
    let mut new_file_content: Vec<u8> = Vec::new();
    file.read_to_end(&mut new_file_content)?;

    assert_eq!(file_content, new_file_content);

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_move_file_when_given_dir_and_filename() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let source_file = tempfile::NamedTempFile::new_in(&dir)?;
    let target_file = dir.path().join("new_name.ext");

    let mut app = helpers::AppBuilder::new()
        .with_file(source_file.path(), None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some(format!(":move {}<ret>", target_file.to_string_lossy()).as_ref()),
        None,
        false,
    )
    .await?;

    assert!(
        target_file.is_file(),
        "target file '{}' should have been created",
        target_file.display()
    );
    assert!(
        !source_file.path().exists(),
        "Source file '{}' should have been removed",
        source_file.path().display()
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_move_file_when_given_dir_only() -> anyhow::Result<()> {
    let source_dir = tempfile::tempdir()?;
    let target_dir = tempfile::tempdir()?;
    let source_file = source_dir.path().join("file.ext");
    std::fs::File::create(&source_file)?;

    let mut app = helpers::AppBuilder::new()
        .with_file(&source_file, None)
        .build()?;

    test_key_sequence(
        &mut app,
        Some(format!(":move {}<ret>", target_dir.path().to_string_lossy()).as_ref()),
        None,
        false,
    )
    .await?;

    let target_file = target_dir.path().join("file.ext");

    assert!(
        target_file.is_file(),
        "target file '{}' should have been created",
        target_file.display()
    );
    assert!(
        !source_file.exists(),
        "Source file '{}' should have been removed",
        source_file.display()
    );

    Ok(())
}
