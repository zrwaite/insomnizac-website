import { gql } from "graphql-request";

export const SKILLS_QUERY = gql`
query {
	skills {
		name
		image
	}
}
`;