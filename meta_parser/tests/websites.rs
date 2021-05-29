use meta_parser::Meta;

#[test]
fn tilman_dev() {
    let html = include_bytes!("tilman.dev.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(meta.title, Some("Click here to accept Cookies".into()));
    assert_eq!(
        meta.description,
        Some("A simple but delicious cookie recipe.".into())
    );
    assert_eq!(meta.image, None);
}

#[test]
fn github_com() {
    let html = include_bytes!("github.com.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(meta.title, Some("rust-lang/rust".into()));
    assert_eq!(
        meta.description,
        Some(
            "Empowering everyone to build reliable and efficient software. - rust-lang/rust".into()
        )
    );
    assert_eq!(meta.image, Some("https://opengraph.githubassets.com/af89e338e57728b027ca005122a73045dcdb2cb6ff879de150fa67bc94f95c37/rust-lang/rust".into()));
}

#[test]
fn thume_ca() {
    let html = include_bytes!("thume.ca.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(
        meta.title,
        Some(
            "Models of Generics and Metaprogramming: Go, Rust, Swift, D and More - Tristan Hume"
                .into()
        )
    );
    assert_eq!(meta.description, None);
    assert_eq!(meta.image, None);
}

#[test]
fn blog_discord_com() {
    let html = include_bytes!("blog.discord.com.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(meta.title, Some("Announcing our Latest Profile Badge: the Certified Discord Moderator | by Nelly | May, 2021 | Discord Blog".into()));
    assert_eq!(meta.description, Some("The Certified Discord Moderator badge rewards the unsung heroes who go above and beyond and make communities on Discord what they are.".into()));
    assert_eq!(
        meta.image,
        Some("https://miro.medium.com/max/1200/1*PyM5eBfN3YXjuZzKgyRa_g.png".into())
    );
}
