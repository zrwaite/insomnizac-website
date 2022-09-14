import { GraphQLClient } from 'graphql-request';
import { dev } from '$app/environment';

const forceProduction = true;

const endpoint = (dev && !forceProduction) ? 'http://localhost:8011/graphql' : 'https://api.insomnizac.xyz/graphql';
export const graphql = new GraphQLClient(endpoint, { headers: {} });
