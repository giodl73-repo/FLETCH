fn assert_contains(source_name: &str, source: &str, needle: &str) {
    assert!(
        source.contains(needle),
        "{source_name} should contain policy text: {needle}"
    );
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
