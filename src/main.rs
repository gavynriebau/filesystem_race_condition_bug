
extern crate filesystem;

use filesystem::*;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_works() {

        // This test captures a bug where the rename function doesn't consistently succeed but
        // instead sometimes panics while attempting to rename the parent directory from
        // "/parent_dir" to "/parent_dir_renamed".
        //
        // I have not been able to verify why this happens but suspect it could be due to a race
        // condition because ~50% of the times I've run the test it passes and the other ~50% of the
        // time running the test fails even though no code modifications have been done.

        // Test setup
        //
        // Make a filesystem as follows:
        //
        // parent_dir
        //    |
        //    +-- child_dir
        //    |     |
        //    |     +-- file_a
        //    |     |
        //    |     +-- file_b
        //    |
        //    +-- file_c
        //
        // The files contain some random bytes which isn't important

        let fs = FakeFileSystem::new();
        let file_a_contents_starting: [u8; 5] = [1_u8, 2_u8, 3_u8, 4_u8, 5_u8];
        let file_b_contents_starting: [u8; 5] = [6_u8, 7_u8, 8_u8, 9_u8, 10_u8];
        let file_c_contents_starting: [u8; 5] = [11_u8, 12_u8, 13_u8, 14_u8, 15_u8];

        fs.create_dir_all(Path::new("parent_dir").join("child_dir")).expect("Failed to create directories");
        fs.create_file(Path::new("parent_dir").join("child_dir").join("file_a"), file_a_contents_starting).expect("Failed to create file_a");
        fs.create_file(Path::new("parent_dir").join("child_dir").join("file_b"), file_b_contents_starting).expect("Failed to create file_b");
        fs.create_file(Path::new("parent_dir").join("file_c"), file_c_contents_starting).expect("Failed to create file_c");

        println!("FS STATE BEFORE RENAMES: {:#?}", fs);

        // These first renames all succeed.
        fs.rename("/parent_dir/file_c", "/parent_dir/file_c_renamed").expect("Failed to rename file_c");
        fs.rename("/parent_dir/child_dir/file_b", "/parent_dir/child_dir/file_b_renamed").expect("Failed to rename file_b");
        fs.rename("/parent_dir/child_dir/file_a", "/parent_dir/child_dir/file_a_renamed").expect("Failed to rename file_a");
        fs.rename("/parent_dir/child_dir", "/parent_dir/child_dir_renamed").expect("Failed to rename child_dir");

        // The last rename fails ~50% of the time.
        fs.rename("/parent_dir", "/parent_dir_renamed").expect("Failed to rename parent_dir");

        println!("FS STATE AFTER RENAMES: {:#?}", fs);
    }
}

