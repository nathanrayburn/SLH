1. Integer overflow
2. CWE-190: Integer Overflow or Wraparound
3. Max centimes / 100


a. Quelle est l'utilité des paramètres --level et --risk ?

Option: --level

By default sqlmap tests all GET parameters and POST parameters. When the value of --level is >= 2 it tests also HTTP Cookie header values. When this value is >= 3 it tests also HTTP User-Agent and HTTP Referer header value for SQL injections. It is however possible to manually specify a comma-separated list of parameter(s) that you want sqlmap to test. This will bypass the dependence on value of --level too.

Option: --risk

This option requires an argument which specifies the risk of tests to perform. There are three risk values. The default value is 1 which is innocuous for the majority of SQL injection points. Risk value 2 adds to the default level the tests for heavy query time-based SQL injections and value 3 adds also OR-based SQL injection tests.