import { render, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useQuery, useMutation } from "picoql";

import ActivityIndicator from "@app/ActivityIndicator";
import ErrorBoundary from "@app/ErrorBoundary";
import Section from "@app/account/Section";

import ManageInvites from "@app/account/ManageInvites";
import ChangeName from "@app/account/ChangeName";
import ChangeEmail from "@app/account/ChangeEmail";

function Account() {
  const { data, loading } = useQuery(graphql`
    query AccountQuery {
      viewer {
        id
        user {
          id
          name
        }
      }
    }
  `);

  return (
    <div class="w-full flex justify-center p-8">
      {
        loading ?
        <ActivityIndicator size="large" /> :
        <div class="w-full max-w-screen-md bg-white dark:bg-gray-800 shadow rounded-lg p-4">
          <h1 class="text-2xl font-semibold leading-none">Account Settings</h1>
          <h2 class="text-xl text-gray-500 mb-4">
            Welcome back {data?.viewer?.user?.name}
          </h2>

          <Section title="Change name" initiallyExpanded={false}>
            <ChangeName currentName={data?.viewer?.user?.name} />
          </Section>
          <Section title="Change email" initiallyExpanded={false}>
            <ChangeEmail />
          </Section>
          <Section title="Invite a friend">
            <ManageInvites />
          </Section>
        </div>
      }
    </div>
  );
}

render(<ErrorBoundary><Account /></ErrorBoundary>, document.getElementById("account"));
