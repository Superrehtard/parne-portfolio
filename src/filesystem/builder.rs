use super::content;
use super::model::VirtualFs;

pub fn build_default_fs() -> VirtualFs {
    let mut fs = VirtualFs::new();

    // Root directory
    fs.add_dir(
        "~",
        &[
            "about.txt",
            "contact.txt",
            "resume.txt",
            "skills",
            "projects",
        ],
    );

    // Root files
    fs.add_file("~/about.txt", content::ABOUT_TEXT);
    fs.add_file("~/contact.txt", content::CONTACT_TEXT);
    fs.add_file("~/resume.txt", content::RESUME_TEXT);

    // Skills directory
    fs.add_dir(
        "~/skills",
        &["languages.txt", "frameworks.txt", "tools.txt"],
    );
    fs.add_file("~/skills/languages.txt", content::SKILLS_LANGUAGES);
    fs.add_file("~/skills/frameworks.txt", content::SKILLS_FRAMEWORKS);
    fs.add_file("~/skills/tools.txt", content::SKILLS_TOOLS);

    // Projects directory
    fs.add_dir("~/projects", &["project1", "project2", "project3"]);

    fs.add_dir("~/projects/project1", &["README.md"]);
    fs.add_file("~/projects/project1/README.md", content::PROJECT1_README);

    fs.add_dir("~/projects/project2", &["README.md"]);
    fs.add_file("~/projects/project2/README.md", content::PROJECT2_README);

    fs.add_dir("~/projects/project3", &["README.md"]);
    fs.add_file("~/projects/project3/README.md", content::PROJECT3_README);

    fs
}
