import { gql } from "graphql-request";

export const PROJECTS_QUERY = gql`
query {
	projects {
		name
		slug
		description
		githubName
		devpostLink
		projectLink
		image
	}
}
`;