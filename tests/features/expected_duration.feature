Feature: Run for an expected duration

  Scenario: Run for an expected duration
   Given an sl command
    When the command is run to completion
    Then the command took at least 3 seconds
     And the exit code is 0
