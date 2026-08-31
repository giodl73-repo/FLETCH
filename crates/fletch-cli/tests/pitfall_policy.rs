fn assert_contains(source_name: &str, source: &str, needle: &str) {
    assert!(
        source.contains(needle),
        "{source_name} should contain policy text: {needle}"
    );
}

#[test]
fn consumer_boundaries_are_machine_readable() {
    let manifest = serde_json::from_str::<serde_json::Value>(include_str!(
        "../../../docs/consumer-boundaries.v1.json"
    ))
    .unwrap();
    assert_eq!(manifest["schema"], "fletch.consumer-boundaries.v1");
    assert_eq!(manifest["owner_repo"], "FLETCH");
    assert_eq!(manifest["authority"]["acquisition"], "FLETCH");
    assert_eq!(
        manifest["authority"]["product_activation"],
        "owning consumer repositories"
    );
    assert_eq!(
        manifest["authority"]["compatibility_acceptance"],
        "affected consumer repositories"
    );

    let boundaries = manifest["boundaries"].as_array().unwrap();
    assert_eq!(boundaries.len(), 3);

    let acquisition = boundaries
        .iter()
        .find(|boundary| boundary["pitfall_id"] == "FLETCH-PF-01")
        .unwrap();
    assert_eq!(acquisition["boundary"], "acquisition_not_activation");
    assert!(acquisition["blocked_claims"].as_array().unwrap().contains(
        &serde_json::Value::String("fetch activates a product view".to_string())
    ));
    assert!(acquisition["activation_requires"]
        .as_array()
        .unwrap()
        .contains(&serde_json::Value::String(
            "consumer-owned domain validation".to_string()
        )));

    let compatibility = boundaries
        .iter()
        .find(|boundary| boundary["pitfall_id"] == "FLETCH-PF-03")
        .unwrap();
    assert_eq!(
        compatibility["boundary"],
        "local_green_not_consumer_compatibility"
    );
    assert!(compatibility["blocked_claims"]
        .as_array()
        .unwrap()
        .contains(&serde_json::Value::String(
            "FLETCH-local tests prove ICELINES compatibility".to_string()
        )));
    assert!(compatibility["compatibility_requires"]
        .as_array()
        .unwrap()
        .contains(&serde_json::Value::String(
            "required downstream rehearsal commands".to_string()
        )));

    let selection = boundaries
        .iter()
        .find(|boundary| boundary["pitfall_id"] == "FLETCH-PF-06")
        .unwrap();
    assert_eq!(selection["boundary"], "selection_not_trust_or_activation");
    assert!(selection["blocked_claims"]
        .as_array()
        .unwrap()
        .contains(&serde_json::Value::String(
            "selected object is trusted by the product".to_string()
        )));
    assert!(selection["handoff_requires"].as_array().unwrap().contains(
        &serde_json::Value::String("product-owned activation command".to_string())
    ));
}

#[test]
fn pitfall_policy_keeps_acquisition_derived_views_and_activation_separate() {
    // FLETCH-PF-01/FLETCH-PF-03/FLETCH-PF-06: consumer-facing docs must keep
    // acquisition, local validation, cache selection, and activation distinct.
    let readme = include_str!("../../../README.md");
    let product_plan = include_str!("../../../PRODUCT_PLAN.md");
    let compatibility = include_str!("../../../docs/compatibility.md");
    let adapter_role = include_str!("../../../.roles/parliament/adapter-boundary-keeper.md");
    let icelines_role = include_str!("../../../.roles/stakeholders/icelines-maintainer.md");
    let release_role = include_str!("../../../.roles/stakeholders/ci-release-engineer.md");

    assert_contains("README.md", readme, "not product activation");
    assert_contains(
        "README.md",
        readme,
        "Fetching is acquisition, not activation",
    );
    assert_contains("README.md", readme, "responsible for domain activation");
    assert_contains(
        "PRODUCT_PLAN.md",
        product_plan,
        "FLETCH registers, resolves, fetches, verifies, bundles",
    );
    assert_contains("PRODUCT_PLAN.md", product_plan, "products interpret NHL");
    assert_contains(
        "PRODUCT_PLAN.md",
        product_plan,
        "FLETCH does not own domain semantics",
    );
    assert_contains(
        "docs/compatibility.md",
        compatibility,
        "consumers own source expansion, parsing, domain validation",
    );
    assert_contains(
        ".roles/parliament/adapter-boundary-keeper.md",
        adapter_role,
        "product-specific rule leak into the core",
    );
    assert_contains(
        ".roles/stakeholders/icelines-maintainer.md",
        icelines_role,
        "NHL",
    );
    assert_contains(
        ".roles/stakeholders/icelines-maintainer.md",
        icelines_role,
        "player/team interpretation",
    );
    assert_contains(
        ".roles/stakeholders/ci-release-engineer.md",
        release_role,
        "CLI smokes need stable, concise output for automation",
    );
}

#[test]
fn pitfall_policy_marks_registry_and_publisher_outputs_as_derived() {
    // FLETCH-PF-02/FLETCH-PF-06: registry and publisher surfaces are searchable
    // read-only views, not ledger authority or product activation.
    let readme = include_str!("../../../README.md");
    let registry_web_test = include_str!("registry_web.rs");
    let publisher_wave =
        include_str!("../../../context/waves/2026-05-15-overwatch-publishers/WAVE.md");
    let manifest_wave =
        include_str!("../../../context/waves/2026-05-16-manifest-maintenance/WAVE.md");

    assert_contains(
        "README.md",
        readme,
        "Publisher commands are read-only derived views",
    );
    assert_contains(
        "README.md",
        readme,
        "registry data, clicking result rows, and inspecting tags",
    );
    assert_contains(
        "crates/fletch-cli/tests/registry_web.rs",
        registry_web_test,
        "FLETCH-PF-02/FLETCH-PF-06",
    );
    assert_contains(
        "context/waves/2026-05-15-overwatch-publishers/WAVE.md",
        publisher_wave,
        "making generated artifacts the source of truth",
    );
    assert_contains(
        "context/waves/2026-05-16-manifest-maintenance/WAVE.md",
        manifest_wave,
        "FLETCH does not own product activation",
    );
}
