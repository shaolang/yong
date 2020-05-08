/*
* Copyright 2020 Shaolang Ai
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

use indoc::indoc;
use plier;
use std::path::PathBuf;

#[test]
fn batch_filename_combines_executables_directory_with_given_filename() {
    let exe_path = PathBuf::from("path/to/binary.exe");
    let actual = plier::batch_filename(exe_path, "batch.sh");
    let mut expected = PathBuf::from("path");
    expected.push("to");
    expected.push("batch.sh");

    assert_eq!(actual, expected);
}

#[test]
fn add_spec_for_new_devkit() {
    let mut actual = plier::load_spec("").unwrap();
    plier::add_spec_entry(&mut actual, "java", vec!["bin".to_string()]);
    let expected = plier::load_spec(indoc!(
        r#"[java]
           bins = ["bin"]
           "#
    ))
    .unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn add_spec_when_others_exists() {
    let mut actual = plier::load_spec("[java]\nbins = [\"bin\"]\n").unwrap();
    plier::add_spec_entry(
        &mut actual,
        "python",
        vec![".".to_string(), "Scripts".to_string()],
    );
    let expected = plier::load_spec(indoc!(
        r#"[java]
           bins = ["bin"]

           [python]
           bins = [".", "Scripts"]
           "#
    ))
    .unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn load_spec_from_empty_string() {
    let actual = plier::load_spec("").unwrap();

    assert_eq!(actual, plier::Spec::new());
}

#[test]
fn load_spec_from_valid_spec_string() {
    let actual = plier::load_spec(indoc!(
        r#"[java]
           bins = ["bin"]

           [python]
           bins = [".", "Scripts"]
           "#
    ))
    .unwrap();
    let mut expected = plier::Spec::new();

    expected.insert(
        "java".to_string(),
        plier::SpecEntry {
            bins: vec!["bin".to_string()],
        },
    );
    expected.insert(
        "python".to_string(),
        plier::SpecEntry {
            bins: vec![".".to_string(), "Scripts".to_string()],
        },
    );

    assert_eq!(actual, expected);
}

#[test]
fn load_spec_from_invalid_spec_string() {
    let actual = plier::load_spec(indoc!(
        r#"[hello]
           bins = ['sbin']

           [world]
           greeting = ['what?']
           "#
    ));

    assert!(actual.is_err());
}
