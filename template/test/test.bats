setup() {
  load 'helper/common-setup'
  _common_setup
}

@test "Can run RTM generate script" {
  run rtm-generate
  assert_success
}
