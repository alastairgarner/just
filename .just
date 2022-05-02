[just]

[lint]
command = "pwd"

    [lint.flake8]
    command = "flake8 ../just"

    [lint.black]
    command = "black ../just --check"

[test]
command = "pwd"

    [test.python]
    command = "pytest"

