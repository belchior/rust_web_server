INSERT INTO users (login, name, email, url, avatar_url, bio, website_url, location, company) VALUES
  ('belchior', 'Belchior Oliveira', 'belchior@email.com', 'https://github.com/belchior', 'https://avatars3.githubusercontent.com/u/2656585?u=de302ff93b129cf3841471deb188a5f5e51a2417&v=4', 'Software engineer', 'https://twitter.com/belchiorso', 'Brasil', 'Belchior Org'),
  ('foo', 'Foo', 'foo@email.com', 'https://github.com/foo', 'https://avatars.githubusercontent.com/u/90864941?v=4', null, null, 'Brasil', 'Foo Company'),
  ('bar', 'Bar', 'bar@email.com', 'https://github.com/bar', 'https://avatars.githubusercontent.com/u/8262550?v=4', 'Bio of Bar', null, 'Brasil', 'Bar Company'),
  ('dee', 'Dee', 'dee@email.com', 'https://github.com/dee', 'https://avatars.githubusercontent.com/u/68281832?v=4', 'Bio of Dee', null, 'Brasil', 'Dee Company');

INSERT INTO organizations (organization_id, login, url, name, description, avatar_url, location)
OVERRIDING SYSTEM VALUE
VALUES
  (9000, 'belchior-org', 'https://github.com/belchior-org', 'My Org', 'Test Org', 'https://avatars3.githubusercontent.com/u/2656585?u=de302ff93b129cf3841471deb188a5f5e51a2417&v=4', 'Brasil'),
  (9001, 'rust-lang', 'https://github.com/rust-lang', 'The Rust Programming Language', 'Empowering everyone to build reliable and efficient software.', 'https://avatars.githubusercontent.com/u/5430905?s=200&v=4', null);

INSERT INTO organizations_members (user_login, organization_login) VALUES
  ('belchior', 'belchior-org'),
  ('foo', 'rust-lang');

INSERT INTO users_following (user_login, following_login, created_at) VALUES
  ('belchior', 'rust-lang', '2026-01-01T00:00:00.001Z'),
  ('belchior', 'bar', '2026-01-01T00:00:00.002Z'),
  ('belchior', 'dee', '2026-01-01T00:00:00.003Z'),
  ('foo', 'belchior', '2026-01-01T00:00:00.005Z'),
  ('foo', 'dee', '2026-01-01T00:00:00.005Z'),
  ('foo', 'rust-lang', '2026-01-01T00:00:00.006Z'),
  ('bar', 'dee', '2026-01-01T00:00:00.007Z'),
  ('bar', 'belchior', '2026-01-01T00:00:00.008Z'),
  ('dee', 'bar', '2026-01-01T00:00:00.009Z'),
  ('dee', 'belchior', '2026-01-01T00:00:00.010Z'),
  ('dee', 'rust-lang', '2026-01-01T00:00:00.011Z');

INSERT INTO languages (language_name, language_color) VALUES
  ('JavaScript','#f1e05a'),
  ('Python','#3572A5'),
  ('Rust', '#dea584'),
  ('Shell','#89e051'),
  ('TypeScript','#2b7489');

INSERT INTO licenses (license_key, license_name) VALUES
  ('unlicense', 'The Unlicense'),
  ('mit', 'MIT License'),
  ('apache-2.0', 'Apache-2.0'),
  ('gpl-2.0', 'GPL-2.0'),
  ('gpl-3.0', 'GPL-3.0');

INSERT INTO repositories (name, repository_id, fork_count, star_count, owner_login, owner_ref, primary_language, url, description, created_at)
OVERRIDING SYSTEM VALUE
VALUES
  ('rust', 10001, 14668, 111452, 'rust-lang', 'organizations', 'Rust', 'https://github.com/rust-lang/rust', 'Empowering everyone to build reliable and efficient software.', '2026-01-01T00:00:00.001Z'),
  ('cargo', 10002, 2994, 14541, 'rust-lang', 'organizations', 'Rust', 'https://github.com/rust-lang/cargo', 'The Rust package manager', '2026-01-01T00:00:00.002Z'),
  ('rust_web_server', 10003, 0, 1, 'belchior', 'users', 'Rust', 'https://github.com/belchior/rust_web_server', 'Project description', '2026-01-01T00:00:00.003Z'),
  ('typescript_web_server', 10004, 0, 0, 'belchior', 'users', 'TypeScript', 'https://github.com/belchior/rust_web_server', 'The purpose of this repository is to practice GraphQL acquired knowledge as well as the ecosystem', '2026-01-01T00:00:00.004Z'),
  ('sql_query_builder', 10005, 8, 73, 'belchior', 'users', 'Rust', 'https://github.com/belchior/rust_web_server', 'Write SQL queries in a simple and composable way', '2026-01-01T00:00:00.005Z'),
  ('repo_foo', 10006, 3, 1, 'belchior-org', 'organizations', 'Rust', 'https://github.com/belchior-org/repo_foo', 'The Foo repository', '2026-01-01T00:00:00.006Z'),
  ('repo_bar', 10007, 7, 2, 'belchior-org', 'organizations', 'TypeScript', 'https://github.com/belchior-org/repo_bar', 'The Bar repository', '2026-01-01T00:00:00.007Z');

INSERT INTO repositories_licenses (repository_id, license_key) VALUES
  (10001, 'mit'),
  (10001, 'apache-2.0'),
  (10001, 'gpl-2.0'),
  (10001, 'gpl-3.0'),
  (10002, 'mit'),
  (10002, 'gpl-2.0'),
  (10003, 'mit'),
  (10004, 'mit'),
  (10005, 'mit'),
  (10006, 'mit'),
  (10007, 'mit');

INSERT INTO repositories_stars (owner_login, repository_id, created_at) VALUES
  ('belchior', 10001, '2026-01-01T00:00:00.001Z'),
  ('belchior', 10002, '2026-01-01T00:00:00.002Z'),
  ('belchior', 10006, '2026-01-01T00:00:00.003Z'),
  ('belchior', 10007, '2026-01-01T00:00:00.004Z'),
  ('bar', 10002, '2026-01-01T00:00:00.005Z');
