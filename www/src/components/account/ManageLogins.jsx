import { h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useQuery } from "picoql";

import ActivityIndicator from "@app/ActivityIndicator";
import LoginSession from "@app/account/LoginSession";

export default function ManageLogins({}) {
  const { data, loading, refetch } = useQuery(graphql`
    query ManageLoginsQuery {
      viewer {
        logins {
          nodes {
            id
            lastUsed
            lastUserAgent {
              name
              operatingSystem
            }
          }
        }
      }
    }
  `);
  const logins = data?.viewer?.logins?.nodes ?? [];

  return (
    <div class="flex flex-col gap-y-2">
      {loading
        ? <ActivityIndicator size="large" style="my-4 mx-auto" />
        : logins.map((login) => <LoginSession {...login} onRevoke={refetch} />)}
    </div>
  );
}
