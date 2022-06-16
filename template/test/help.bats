setup() {
  load 'helper/common-setup'
  _common_setup
}

@test "RTM help" {
  run rtm-generate --help

  assert_success

  assert_output --partial "Usage:"
  assert_output --partial "    rt-generate [OPTIONS] <SUBCOMMAND>"
  assert_output --partial "OPTIONS:"
  assert_output --partial "    -h|--help          Display this help"
  assert_output --partial "    -m|--monochrome    Disable colour output"
  assert_output --partial "    -q|--quiet         Run silently unless an error occurs"
  assert_output --partial "    -v|--verbose       Display verbose output"
  assert_output --partial "SUBCOMMAND:"
  assert_output --partial "    base      New project using the base template"
  assert_output --partial "    domain    New feature/domain within a project"
}
