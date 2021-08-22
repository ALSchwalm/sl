Feature: Legacy options should be supported

  Scenario: Run with the specific C51 train
   Given an sl command line of "-c"
    When the command is run to completion
    Then the exit code is 0

  Scenario: Run with the 'logo' train
   Given an sl command line of "-l"
    When the command is run to completion
    Then the exit code is 0

  Scenario: Run with the flying flag
   Given an sl command line of "-F"
    When the command is run to completion
    Then the exit code is 0

  Scenario: Run with the accident flag
   Given an sl command line of "-a"
    When the command is run to completion
    Then the exit code is 0

  Scenario: The flying flag should compose with others
   Given an sl command line of "-Fc"
    When the command is run to completion
    Then the exit code is 0