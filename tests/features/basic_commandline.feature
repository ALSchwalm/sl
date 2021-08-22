Feature: Basic commandline usage

  Scenario: Run and get help text
   Given an sl command line of "--help"
    When the command is run to completion
    Then the exit code is 0

  Scenario: Run with unknown flag
   Given an sl command line of "--nonexist"
    When the command is run to completion
    Then the exit code is 1
