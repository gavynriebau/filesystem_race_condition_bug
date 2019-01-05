

# Rename Bug (possible race condition?)

**_This issue has been fixed already as of version "0.4.3" of the `filesystem` create_**


This test captures a bug where the rename function in the rust `filesystem` crate doesn't consistently succeed but instead _sometimes_ panics while attempting to rename the parent directory from
"/parent_dir" to "/parent_dir_renamed".

I haven't verified why this happens but thought it might be due to a race
condition because around half the time I run `cargo test` the test passes but the other half of the
time it fails _even though no code modifications have been done between subsequent executions_.

The test makes a filesystem with the following layout and then attempts to rename files and directories:

```rust
// parent_dir
//    |
//    +-- child_dir
//    |     |
//    |     +-- file_a
//    |     |
//    |     +-- file_b
//    |
//    +-- file_c
```

I've been running the test using `cargo test` (repeat this command many times until you see it fail).

Here's an example of the test succeeding after running `cargo test`:
```bash
gavyn@orac:~/Code/github.com/filesystem_race_condition_bug$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running target/debug/deps/filesystem_race_condition_bug-fb348f0e5ac20289

running 1 test
test tests::rename_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Here's an example of the test failing after running `cargo test` even though the code hasn't been changed at all:

```bash
gavyn@orac:~/Code/github.com/filesystem_race_condition_bug$ cargo test                                                                                               [44/1973]
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running target/debug/deps/filesystem_race_condition_bug-fb348f0e5ac20289

running 1 test
test tests::rename_works ... FAILED

failures:

---- tests::rename_works stdout ----
FS STATE BEFORE RENAMES: FakeFileSystem {
    registry: Mutex {
        data: Registry {
            cwd: "/",
            files: {
                "/parent_dir/child_dir/file_a": File(
                    File {
                        contents: [
                            1,
                            2,
                            3,
                            4,
                            5
                        ],
                        mode: 420
                    }
                ),
                "/parent_dir/child_dir/file_b": File(
                    File {
                        contents: [
                            6,
                            7,
                            8,
                            9,
                            10
                        ],
                        mode: 420
                    }
                ),
                "/parent_dir/child_dir": Dir(
                    Dir {
                        mode: 420
                    }
                ),
                "/parent_dir": Dir(
                    Dir {
                        mode: 420
                    }
                ),
                "/": Dir(
                    Dir {
                        mode: 420
                    }
                ),
                "/parent_dir/file_c": File(
                    File {
                        contents: [
                            11,
                            12,
                            13,
                            14,
                            15
                        ],
                        mode: 420
                    }
                )
            }
        }
    }
}
thread 'tests::rename_works' panicked at 'Failed to rename parent_dir: Custom { kind: NotFound, error: StringError("entity not found") }', libcore/result.rs:1009:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::rename_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--bin filesystem_race_condition_bug'
```