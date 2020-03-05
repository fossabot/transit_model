// Copyright (C) 2017 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>

use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};
use transit_model::{apply_rules, test_utils::*};

fn compare_report(report_path: PathBuf, fixture_report_output: PathBuf) {
    let output_contents = get_file_content(report_path);
    let expected_contents = get_file_content(fixture_report_output);
    assert_eq!(expected_contents, output_contents);
}

fn test_apply_rules(
    cc_rules_dir: &str,
    p_rules_dir: &str,
    n_consolidation: &str,
    fixture_output_dir: &str,
    fixture_report_output: &str,
) {
    test_in_tmp_dir(|path| {
        let mut file_to_compare = vec![
            "commercial_modes.txt",
            "equipments.txt",
            "geometries.txt",
            "lines.txt",
            "networks.txt",
            "physical_modes.txt",
            "routes.txt",
            "stops.txt",
            "ticket_use_perimeters.txt",
            "trips.txt",
            "trip_properties.txt",
        ];

        let input_dir = "tests/fixtures/apply_rules/input";

        let mut cc_rules: Vec<PathBuf> = vec![];
        if !cc_rules_dir.is_empty() {
            cc_rules.push(Path::new(cc_rules_dir).to_path_buf());
            file_to_compare.push("object_codes.txt")
        }

        let mut p_rules: Vec<PathBuf> = vec![];
        if !p_rules_dir.is_empty() {
            p_rules.push(Path::new(p_rules_dir).to_path_buf());
        }

        let consolidation = if n_consolidation.is_empty() {
            None
        } else {
            Some(Path::new(n_consolidation).to_path_buf())
        };

        let report_path = path.join("report.json");
        let model = apply_rules::apply_rules(
            transit_model::ntfs::read(input_dir).unwrap(),
            cc_rules,
            p_rules,
            consolidation,
            Path::new(&report_path).to_path_buf(),
        )
        .unwrap();

        transit_model::ntfs::write(&model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(&path, Some(file_to_compare), fixture_output_dir);
        compare_report(report_path, Path::new(fixture_report_output).to_path_buf());
    });
}
#[test]
fn test_no_property_rules() {
    test_apply_rules(
        "",
        "",
        "",
        "./tests/fixtures/apply_rules/output",
        "./tests/fixtures/apply_rules/output_report/report.json",
    );
}

#[test]
fn test_apply_complementary_codes() {
    test_apply_rules(
        "./tests/fixtures/apply_rules/complementary_codes_rules.txt",
        "",
        "",
        "./tests/fixtures/apply_rules/output_apply_complementary_codes",
        "./tests/fixtures/apply_rules/output_report/report_apply_complementary_codes.json",
    );
}

#[test]
fn test_apply_property() {
    test_apply_rules(
        "./tests/fixtures/apply_rules/complementary_codes_rules.txt",
        "./tests/fixtures/apply_rules/property_rules.txt",
        "",
        "./tests/fixtures/apply_rules/output_apply_property",
        "./tests/fixtures/apply_rules/output_report/report_apply_property.json",
    );
}

#[test]
fn test_ntw_consolidation() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation.json",
        "./tests/fixtures/apply_rules/output_consolidation",
        "./tests/fixtures/apply_rules/output_report/report.json",
    );
}

#[test]
#[should_panic]
fn test_ntw_consolidation_unvalid() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_unvalid.json",
        "",
        "",
    );
}

#[test]
fn test_ntw_consolidation_with_object_code() {
    test_apply_rules(
        "./tests/fixtures/apply_rules/complementary_codes_rules.txt",
        "./tests/fixtures/apply_rules/property_rules.txt",
        "./tests/fixtures/apply_rules/ntw_consolidation.json",
        "./tests/fixtures/apply_rules/output_consolidation_with_object_code",
        "./tests/fixtures/apply_rules/output_report/report_consolidation_with_object_code.json",
    );
}

#[test]
fn test_ntw_consolidation_2_ntw() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_2_ntw.json",
        "./tests/fixtures/apply_rules/output_consolidation_2_ntw",
        "./tests/fixtures/apply_rules/output_report/report.json",
    );
}

#[test]
fn test_ntw_consolidation_2_diff_ntw() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_2_diff_ntw.json",
        "./tests/fixtures/apply_rules/output_consolidation_2_diff_ntw",
        "./tests/fixtures/apply_rules/output_report/report.json",
    );
}

#[test]
fn test_ntw_consolidation_unknown_id() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_unknown_id.json",
        "./tests/fixtures/apply_rules/output",
        "./tests/fixtures/apply_rules/output_report/report_consolidation_unknown_id.json",
    );
}

#[test]
#[should_panic]
fn test_ntw_consolidation_duplicate_id() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_duplicate_id.json",
        "",
        "",
    );
}

#[test]
#[should_panic]
fn test_ntw_consolidation_unvalid_network() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_unvalid_network.json",
        "",
        "",
    );
}

#[test]
fn test_ntw_consolidation_no_grouped_from() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_no_grouped_from.json",
        "./tests/fixtures/apply_rules/output",
        "./tests/fixtures/apply_rules/output_report/report_consolidation_no_grouped_from.json",
    );
}

#[test]
fn test_ntw_consolidation_empty_grouped_from() {
    test_apply_rules(
        "",
        "",
        "./tests/fixtures/apply_rules/ntw_consolidation_empty_grouped_from.json",
        "./tests/fixtures/apply_rules/output",
        "./tests/fixtures/apply_rules/output_report/report_consolidation_empty_grouped_from.json",
    );
}

#[test]
fn report_missing_geometry() {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };
    fn write_property_rules_file<P: AsRef<Path>>(filepath: P) {
        let property_rules_content = r#"object_type,object_id,property_name,property_old_value,property_value
line,line_id,line_geometry,"LINESTRING(10.1 20.2,30.3 40.4)","LINESTRING(40.1 30.2,20.3 10.4)""#;
        let property_rules_file = File::create(filepath).unwrap();
        let mut writer = BufWriter::new(&property_rules_file);
        write!(&mut writer, "{}", property_rules_content).unwrap();
    }
    test_in_tmp_dir(|path| {
        let property_rules_filepath = path.join("property_rules.txt");
        write_property_rules_file(&property_rules_filepath);
        crate::apply_rules::property_rule::apply_rules
    });
}
