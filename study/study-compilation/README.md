# Useful commands

`https://youtu.be/lzKeecy4OmQ?t=17936`
`https://learn.microsoft.com/en-us/training/paths/rust-first-steps/`
`https://learn.microsoft.com/en-us/training/modules/introduction-to-github-actions/`
`https://learn.microsoft.com/en-us/training/modules/github-actions-automate-tasks/`
`https://learn.microsoft.com/en-us/training/modules/github-actions-ci/`

## Cargo

- init

  ```bash
    $ cargo init
  ```

- run

  - Default:

    ```bash
      $ cargo run
    ```

  - Omit compile debug:

    ```bash
      $ cargo run -q
    ```

  - Running a binary:

    ```bash
      $ cargo run --bin name-of-the-binary # You can combine with -q as well
    ```

    _You don't need to specify the file extension .rs_
