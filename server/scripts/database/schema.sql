CREATE TABLE IF NOT EXISTS users (
  user_id           BIGINT GENERATED ALWAYS AS IDENTITY,
  login             VARCHAR NOT NULL,
  name              VARCHAR,
  avatar_url        VARCHAR NOT NULL,
  bio               VARCHAR,
  company           VARCHAR,
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  email             VARCHAR CHECK(LENGTH(email) >= 3) NOT NULL,
  location          VARCHAR,
  url               VARCHAR CHECK(LENGTH(url) >= 5) NOT NULL,
  website_url       VARCHAR CHECK(LENGTH(website_url) >= 5),
  PRIMARY KEY(user_id),
  UNIQUE(login)
);

CREATE TABLE IF NOT EXISTS organizations (
  organization_id   BIGINT GENERATED ALWAYS AS IDENTITY,
  login             VARCHAR NOT NULL,
  name              VARCHAR,
  avatar_url        VARCHAR NOT NULL,
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  description       VARCHAR,
  email             VARCHAR CHECK(LENGTH(email) >= 3),
  location          VARCHAR,
  url               VARCHAR CHECK(LENGTH(url) >= 5) NOT NULL,
  website_url       VARCHAR CHECK(LENGTH(website_url) >= 5),
  PRIMARY KEY(organization_id),
  UNIQUE(login)
);

CREATE TABLE IF NOT EXISTS licenses (
  license_key      VARCHAR,
  license_name     VARCHAR,
  PRIMARY KEY(license_key)
);

CREATE TABLE IF NOT EXISTS languages (
  language_color    VARCHAR CHECK(LENGTH(language_color) >= 4) NOT NULL,
  language_name     VARCHAR,
  PRIMARY KEY(language_name)
);

CREATE TABLE IF NOT EXISTS repositories (
  repository_id     BIGINT GENERATED ALWAYS AS IDENTITY,
  name              VARCHAR NOT NULL,
  owner_login       VARCHAR NOT NULL,
  owner_ref         VARCHAR NOT NULL,
  description       VARCHAR,
  fork_count        INTEGER NOT NULL DEFAULT 0 CHECK(fork_count >= 0),
  star_count        INTEGER NOT NULL DEFAULT 0 CHECK(star_count >= 0),
  primary_language  VARCHAR REFERENCES languages(language_name),
  url               VARCHAR NOT NULL CHECK(LENGTH(url) >= 5),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT NOW(),
  PRIMARY KEY(repository_id),
  UNIQUE(name, owner_login, owner_ref)
);

CREATE TABLE IF NOT EXISTS repositories_licenses (
  repository_id     BIGINT REFERENCES repositories(repository_id),
  license_key       VARCHAR REFERENCES licenses(license_key),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(repository_id, license_key)
);

CREATE TABLE IF NOT EXISTS repositories_stars (
  owner_login       VARCHAR NOT NULL,
  repository_id     BIGINT REFERENCES repositories(repository_id),
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(owner_login, repository_id)
);

CREATE TABLE IF NOT EXISTS users_following (
  user_login        VARCHAR REFERENCES users(login),
  following_login   VARCHAR NOT NULL,
  created_at        TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(user_login, following_login)
);

CREATE TABLE IF NOT EXISTS organizations_members (
  organization_login VARCHAR REFERENCES organizations(login),
  user_login         VARCHAR REFERENCES users(login),
  created_at         TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(organization_login, user_login)
);
