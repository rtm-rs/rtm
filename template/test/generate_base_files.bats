setup() {
  load 'helper/common-setup'
  _common_setup
}

@test "Generate base files" {
    load rtm-generate
    RTM_TPL_VERBOSE=true

    rm -rf "${RTM_TEMPLATE_DATA}"
    git clone --depth 1 -b main "${RTM_TEMPLATE_URL}" "${RTM_TEMPLATE_DATA}"

    run generate_base_files

    assert_success
    assert_dir_exists "${RTM_TPL_PARENT_DIR}/${RTM_TPL_PROJECT_NAME}"

    rm -rf "${RTM_TEMPLATE_DATA}"
}
