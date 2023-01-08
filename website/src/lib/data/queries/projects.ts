import { gql } from "graphql-request";

export const PROJECTS_QUERY = gql`
query {
	projects {
		name
		slug
		description
		githubLink
		devpostLink
		projectLink
		image
		featured
		skills {
			name
			image
		}
	}
}
`;