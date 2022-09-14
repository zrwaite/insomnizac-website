import { GraphQLClient } from 'graphql-request';
import { dev } from '$app/environment';

const endpoint = (dev && !"a") ? 'http://localhost:8011/graphql' : 'https://api.insomnizac.xyz/graphql';
export const graphql = new GraphQLClient(endpoint, { headers: {} });
