Feature: Select a specific train

  Scenario: Run with a valid specific train
   Given an sl command line of "-n 1"
    When the command is run to completion
    Then the exit code is 0

  Scenario: Run with higher number than available
   Given an sl command line of "-n 100"
    When the command is run to completion
    Then the exit code is 1

  Scenario: Run with an invalid specific train
   Given an sl command line of "-n asdf"
    When the command is run to completion
    Then the exit code is 1