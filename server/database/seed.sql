INSERT INTO users (login, name, email, url, avatar_url, bio, website_url, location) VALUES
  ('belchior', 'Belchior Oliveira', 'belchior@email.com', 'https://github.com/belchior', 'https://avatars3.githubusercontent.com/u/2656585?u=de302ff93b129cf3841471deb188a5f5e51a2417&v=4', 'Software developer', 'https://twitter.com/belchiorso', 'Brasil'),
  ('brson', 'Brian Anderson', 'brson@email.com', 'https://github.com/brson', 'https://avatars.githubusercontent.com/u/147214?v=4', null, null, null),
  ('bar', 'Bar', 'bar@email.com', 'https://github.com/bar', 'https://bar.com/avatar.jpg', 'Bio of Bar', null, null),
  ('dee', 'Dee', 'dee@email.com', 'https://github.com/bar', 'https://dee.com/avatar.jpg', 'Bio of Dee', null, null);

INSERT INTO users_following (user_login, following_login) VALUES
  ('belchior', 'brson'),
  ('bar', 'dee'),
  ('dee', 'bar');

INSERT INTO organizations (login, url, name, description, avatar_url) VALUES
  ('rust-lang', 'https://github.com/rust-lang', 'The Rust Programming Language', 'Empowering everyone to build reliable and efficient software.', 'https://avatars.githubusercontent.com/u/5430905?s=200&v=4');

INSERT INTO users_organizations (organization_login, user_login) VALUES
  ('rust-lang', 'brson');

INSERT INTO languages (language_name, language_color) VALUES
  ('JavaScript','#f1e05a'),
  ('Python','#3572A5'),
  ('Rust', '#dea584'),
  ('Shell','#89e051'),
  ('TypeScript','#2b7489');

INSERT INTO licenses (license_name) VALUES
  ('MIT License');

INSERT INTO repositories (name, fork_count, owner_login, owner_ref, primary_language, url, description) VALUES
  ('rust', 8612, 'rust-lang', 'organizations', 'Rust', 'https://github.com/rust-lang/rust', 'Empowering everyone to build reliable and efficient software.'),
  ('cargo', 1583, 'rust-lang', 'organizations', 'Rust', 'https://github.com/rust-lang/cargo', 'The Rust package manager'),
  ('rust_web_server', 0, 'belchior', 'users', 'Rust', 'https://github.com/belchior/rust_web_server', null);

INSERT INTO users_starred_repositories (user_login, repository_name) VALUES
  ('belchior', 'rust'),
  ('belchior', 'cargo'),
  ('bar', 'cargo');
