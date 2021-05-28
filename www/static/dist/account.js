import {render, h} from "preact";
import {useState} from "preact/hooks";
import {graphql, useQuery, useMutation} from "./picoql.js";
import ActivityIndicator from "./components/ActivityIndicator.js";
import ErrorBoundary from "./components/ErrorBoundary.js";
import Section from "./components/account/Section.js";
import ChangeEmail from "./components/account/ChangeEmail.js";
function Account() {
  const {data, loading} = useQuery(graphql`
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
  return /* @__PURE__ */ h("div", {
    class: "w-full flex justify-center p-8"
  }, loading ? /* @__PURE__ */ h(ActivityIndicator, {
    size: "large"
  }) : /* @__PURE__ */ h("div", {
    class: "w-full max-w-screen-md bg-white dark:bg-gray-800 shadow rounded-lg p-4"
  }, /* @__PURE__ */ h("h1", {
    class: "text-2xl font-semibold"
  }, "Account Settings"), /* @__PURE__ */ h("h2", {
    class: "text-xl text-gray-500 mb-4"
  }, "Welcome back ", data?.viewer?.user?.name), /* @__PURE__ */ h(Section, {
    title: "Invite a friend"
  }, "TODO"), /* @__PURE__ */ h(Section, {
    title: "Change email",
    initiallyExpanded: false
  }, /* @__PURE__ */ h(ChangeEmail, null))));
}
render(/* @__PURE__ */ h(ErrorBoundary, null, /* @__PURE__ */ h(Account, null)), document.getElementById("account"));
