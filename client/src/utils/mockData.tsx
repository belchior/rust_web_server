import { Organization, Repository, User } from './interfaces';

export const profileOwnerUser: User = {
  __typename: 'User',
  avatarUrl: 'path/to/avatarUrl.png',
  bio: 'Software developer',
  company: 'Company',
  email: 'belchior@email.com',
  id: '48ce',
  location: 'Brazil',
  login: 'belchior',
  name: 'Belchior Oliveira',
  url: '/belchior',
  websiteUrl: 'https://github.com/belchior',
};

export const organization: Organization = {
  __typename: 'Organization',
  avatarUrl: 'path/to/avatarUrl.png',
  description: 'Ecma International, Technical Committee 39 - ECMAScript',
  id: '1234',
  location: 'The web',
  login: 'tc39',
  name: 'Ecma TC39',
  people: {
    edges: [{ node: profileOwnerUser }],
    pageInfo: { endCursor: '', hasNextPage: false, hasPreviousPage: false, startCursor: '', }
  },
  url: 'https://github.com/tc39',
  websiteUrl: 'https://www.ecma-international.org/memento/tc39-rf-tg.htm',
};

export const repository: Repository = {
  __typename: 'Repository',
  description: 'repository description',
  forkCount: 123,
  id: 'ba86',
  licenseInfo: {
    name: 'MIT',
  },
  name: 'learning-graphql',
  owner: profileOwnerUser,
  primaryLanguage: {
    color: '#f1e05a',
    name: 'JavaScript'
  },
  url: 'https://github.com/belchior/learning-graphql',
};

export const user: User = {
  __typename: 'User',
  avatarUrl: 'path/to/avatarUrl.png',
  bio: 'Software developer',
  email: 'belchior@email.com',
  followers: {
    edges: [{ node: profileOwnerUser }],
    pageInfo: { endCursor: '', hasNextPage: false, hasPreviousPage: false, startCursor: '', }
  },
  following: {
    edges: [{ node: profileOwnerUser }],
    pageInfo: { endCursor: '', hasNextPage: false, hasPreviousPage: false, startCursor: '', }
  },
  id: '48ce',
  login: 'belchior',
  name: 'Belchior Oliveira',
  organizations: {
    edges: [{ node: organization }],
    pageInfo: { endCursor: '', hasNextPage: false, hasPreviousPage: false, startCursor: '', }
  },
  repositories: {
    edges: [{ node: repository }],
    pageInfo: { endCursor: '', hasNextPage: false, hasPreviousPage: false, startCursor: '', }
  },
  starredRepositories: {
    edges: [{ node: repository }],
    pageInfo: { endCursor: '', hasNextPage: false, hasPreviousPage: false, startCursor: '', }
  },
  url: '/belchior',
  websiteUrl: 'https://github.com/belchior',
};

