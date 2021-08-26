use meta_parser::Meta;

#[test]
fn tilman_dev() {
    let html = include_bytes!("html/tilman.dev.html");
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
    let html = include_bytes!("html/github.com.html");
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
    let html = include_bytes!("html/thume.ca.html");
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
    let html = include_bytes!("html/blog.discord.com.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(
        meta.title,
        Some("Announcing our Latest Profile Badge: the Certified Discord Moderator".into())
    );
    assert_eq!(meta.description, Some("The Certified Discord Moderator badge rewards the unsung heroes who go above and beyond and make communities on Discord what they are.".into()));
    assert_eq!(
        meta.image,
        Some("https://miro.medium.com/max/1200/1*PyM5eBfN3YXjuZzKgyRa_g.png".into())
    );
}

#[test]
fn dev_to() {
    let html = include_bytes!("html/dev.to.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(
        meta.title,
        Some("Buckle Up For a Wild Decade in Cloud Computing".into())
    );
    assert_eq!(meta.description, Some("I’m sure you were affected by the Fastly outage yesterday. The company responded quickly, and it wasn...".into()));
    assert_eq!(
        meta.image,
        Some("https://res.cloudinary.com/practicaldev/image/fetch/s--yJJt-vDA--/c_imagga_scale,f_auto,fl_progressive,h_500,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/z7hv4g9221qw4qq5wi56.jpeg".into())
    );
}

#[test]
fn www_nushell_sh() {
    let html = include_bytes!("html/www.nushell.sh.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(meta.title, Some("Nushell".into()));
    assert_eq!(meta.description, Some("A new type of shell.".into()));
    assert_eq!(meta.image, None);
}

#[test]
fn soatok_blog() {
    let html = include_bytes!("html/soatok.blog.html");
    let mut meta = Meta::new();
    meta.parse(html);
    assert_eq!(
        meta.title,
        Some("Programmers Don’t Understand Hash\u{a0}Functions".into())
    );
    assert_eq!(meta.description, None);
    assert_eq!(
        meta.image,
        Some(
            "https://soatok.files.wordpress.com/2021/08/blogheader-hashfunctions.png?w=640".into()
        )
    );
}
