
CREATE TABLE Questions (
	Id INTEGER PRIMARY KEY AUTOINCREMENT,
	Question TEXT,
	Password TEXT
);

INSERT INTO Questions (Question, Password)
VALUES ("Rust was designed by?", "Graydon Hoare");

INSERT INTO Questions (Question, Password)
VALUES ("Rust can be used in web....?", "assembly");

INSERT INTO Questions (Question, Password)
VALUES ("It enables Rust to make memory safety guarantees without needing a garbage collector. What is it?",
"Ownership");

INSERT INTO Questions (Question, Password)
VALUES ("One of Rust license.", "MIT");

INSERT INTO Questions (Question, Password)
VALUES ("Useful crate for cli apliactions.", "clap");

INSERT INTO Questions (Question, Password)
VALUES ("Rust's build system and package manager.", "Cargo");

INSERT INTO Questions (Question, Password)
VALUES ("Operating system wriiten in Rust", "Redox");

INSERT INTO Questions (Question, Password)
VALUES ("Infinitee loop in Rust.", "loop");

INSERT INTO Questions (Question, Password)
VALUES ("Collection of methods defined for an unknown type.", "trait");

INSERT INTO Questions (Question, Password)
VALUES ("Data-driven game engine written in Rust.", "Amethyst");