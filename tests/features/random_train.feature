Feature: Choose a random train from a directory

  Scenario: Run with a directory of valid trains
   Given an sl command line of "-d tests/assets/valid"
    When the command is run to completion
    Then the command took at least 3 seconds
     And the exit code is 0

  Scenario: Run with an empty directory
   Given an sl command line of "-d tests/assets/invalid"
    When the command is run to completion
    Then the exit code is 1

  Scenario: Attempt to use specific selection and random directory
   Given an sl command line of "-d tests/assets/valid -n 1"
    When the command is run to completion
    Then the exit code is 1