import { GraphQLClient } from 'graphql-request';
import { dev } from '$app/environment';

const endpoint = dev ? 'http://localhost:3000/graphql' : 'https://api.insomnizac.xyz/graphql';
export const graphql = new GraphQLClient(endpoint, { headers: {} });
