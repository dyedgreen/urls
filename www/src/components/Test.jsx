import { h } from "preact";
import { graphql, useQuery } from "picoql";

export default function Test() {
  const { data, loading } = useQuery(graphql`
    query Test {
      viewer {
        user {
          name
        }
      }
    }
  `);
  if (!loading) {
    return <h1>Name: {data?.viewer?.user?.name}</h1>;
  } else {
    return <h1>Loading ...</h1>;
  }
}
