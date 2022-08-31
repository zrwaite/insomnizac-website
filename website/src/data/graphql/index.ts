import { GraphQLClient } from 'graphql-request'

export const graphql = new GraphQLClient('http://localhost:8011/graphql', { headers: {} })