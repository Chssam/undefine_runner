### Setup Static Website to run in GitHub using [Dioxus](https://dioxuslabs.com/)

Pre-require

-  Have [Rust](https://rust-lang.org/) installed

-  Follow up installing [Dioxus guide](https://dioxuslabs.com/learn/0.7/getting_started/)

-  [Dioxus Setup app](https://dioxuslabs.com/learn/0.7/tutorial/new_app)

1. Ensure inside "Dioxus.toml", change "base_path" to the name of your GitHub repo

2. Run following commands
```bash
dx bundle --release --out-dir docs
```

3. Take all item inside public to "docs"

4. Drop "docs" folder directly to GitHub drop file zone

5. In your repo, -> Settings -> Pages

6. Follow below image

![image](https://github.com/Chssam/underfine_runner/blob/main/Showcase/Screenshot%202026-05-26%20111320.png?raw=true)

7. Click save

8. Wait for it to deploy

9. Profit
