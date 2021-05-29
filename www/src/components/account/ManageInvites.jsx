import { h, Fragment } from "preact";
import { useState } from "preact/hooks";
import { graphql, useQuery, useMutation } from "picoql";

import ActivityIndicator from "@app/ActivityIndicator";
import Button from "@app/Button";
import Notice from "@app/Notice";
import Invite from "@app/account/Invite";

export default function ManageInvites({}) {
  const { data, loading, refetch } = useQuery(graphql`
    query ManageInvitesQuery {
      viewer {
        invites {
          edges {
            node {
              id
              token
              claimedBy {
                id
                name
              }
            }
          }
        }
      }
    }
  `);
  const invites = data?.viewer?.invites?.edges?.map(({ node }) => node) ?? [];

  const [error, setError] = useState(null);
  const { commit, inFlight } = useMutation(graphql`
    mutation IssueInvite {
      issueInvite {
        id
      }
    }
  `, {
    onCommit: refetch,
    onError: ([{message}]) => setError(message),
  });

  return (
    <>
      {loading ? <ActivityIndicator size="large" style="my-4 mx-auto" /> : invites.map(invite => <Invite {...invite} />)}
      {error && <Notice message={error} type="error" style="my-2" />}
      <Button
        title="Issue new Invitation"
        type="flat"
        onClick={commit}
        loading={inFlight}
        disabled={inFlight}
        style="w-full"
      />
    </>
  );
}
