CREATE TABLE IF NOT EXISTS users (
  avatar_url        VARCHAR(500) NOT NULL,
  bio               VARCHAR(500),
  company           VARCHAR(120),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  email             VARCHAR(120) check(length(email) >= 3) NOT NULL,
  id                SERIAL,
  location          VARCHAR(120),
  login             VARCHAR(120) NOT NULL,
  name              VARCHAR(120),
  url               VARCHAR(500) check(length(url) >= 5) NOT NULL,
  website_url       VARCHAR(500) check(length(website_url) >= 5),
  PRIMARY KEY(id)
);
CREATE UNIQUE INDEX ON users (login);


CREATE TABLE IF NOT EXISTS organizations (
  avatar_url        VARCHAR(500) NOT NULL,
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  description       VARCHAR(500),
  email             VARCHAR(120) check(length(email) >= 3),
  id                SERIAL,
  location          VARCHAR(120),
  login             VARCHAR(120) NOT NULL,
  name              VARCHAR(120),
  url               VARCHAR(500) check(length(url) >= 5) NOT NULL,
  website_url       VARCHAR(500) check(length(website_url) >= 5),
  PRIMARY KEY(id)
);
CREATE UNIQUE INDEX ON organizations (login);


CREATE TABLE IF NOT EXISTS licenses (
  license_name     VARCHAR(120),
  PRIMARY KEY(license_name)
);

CREATE TABLE IF NOT EXISTS languages (
  language_color    VARCHAR(7) check(length(language_color) >= 4) NOT NULL,
  language_name     VARCHAR(120),
  PRIMARY KEY(language_name)
);

CREATE TABLE IF NOT EXISTS repositories (
  description       VARCHAR(500),
  fork_count        INTEGER NOT NULL CHECK(fork_count >= 0),
  id                SERIAL,
  license_name      VARCHAR(120) REFERENCES licenses(license_name),
  name              VARCHAR(120) NOT NULL,
  owner_login       VARCHAR(120) NOT NULL,
  owner_ref         VARCHAR(40) NOT NULL,
  primary_language  VARCHAR(120) REFERENCES languages(language_name),
  url               VARCHAR(500) NOT NULL CHECK(LENGTH(url) >= 5),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT NOW(),
  PRIMARY KEY(id)
);
CREATE UNIQUE INDEX ON repositories (name);
CREATE UNIQUE INDEX ON repositories (name, owner_login, owner_ref);


CREATE TABLE IF NOT EXISTS users_following (
  user_login        VARCHAR(120) REFERENCES users(login),
  following_login   VARCHAR(120) REFERENCES users(login),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(user_login, following_login)
);

CREATE TABLE IF NOT EXISTS users_organizations (
  user_login         VARCHAR(120) REFERENCES users(login),
  organization_login VARCHAR(120) REFERENCES organizations(login),
  created_at         TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(organization_login, user_login)
);

CREATE TABLE IF NOT EXISTS users_starred_repositories (
  user_login        VARCHAR(120) REFERENCES users(login),
  repository_name   VARCHAR(120) REFERENCES repositories(name),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(user_login, repository_name)
);
