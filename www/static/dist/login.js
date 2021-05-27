import {render, h} from "preact";
import {useState} from "preact/hooks";
import {graphql, useMutation} from "./picoql.js";
import ErrorBoundary from "./components/ErrorBoundary.js";
import TextInput from "./components/TextInput.js";
import Button from "./components/Button.js";
import Notice from "./components/Notice.js";
function Login() {
  const [email, setEmail] = useState("");
  const [code, setCode] = useState("");
  const [showCode, setShowCode] = useState(false);
  const [error, setError] = useState(null);
  const [notice, setNotice] = useState(null);
  const request = useMutation(graphql`
    mutation RequestLogin($email: String!) {
      requestLogin(email: $email) {
        ok
      }
    }
  `, {
    onCommit: () => {
      setNotice(`A login code was sent to ${email}.`);
      setError(null);
      setShowCode(true);
    },
    onError: (errors) => {
      setError(`Could not request code: ${errors[0].message}`);
    }
  });
  const commitRequest = () => request.commit({email});
  const login = useMutation(graphql`
    mutation Login($email: String!, $code: String!) {
      login(email: $email, token: $code)
    }
  `, {
    onCommit: ({login: login2}) => {
      document.cookie = `${window.__auth_cookie}=${login2};path=/;max-age=604800`;
      window.location.href = "/";
    },
    onError: (errors) => {
      setError(`Failed to log in: ${errors[0].message}`);
    }
  });
  const commitLogin = () => login.commit({email, code});
  const loading = request.inFlight || login.inFlight;
  const submit = (e) => {
    e.preventDefault();
    if (showCode) {
      commitLogin();
    } else {
      commitRequest();
    }
  };
  return /* @__PURE__ */ h(ErrorBoundary, null, /* @__PURE__ */ h("div", {
    class: "w-full flex justify-center p-8"
  }, /* @__PURE__ */ h("form", {
    class: "w-full max-w-md bg-white shadow rounded-lg p-4",
    onSubmit: submit
  }, /* @__PURE__ */ h("h1", {
    class: "text-2xl font-semibold"
  }, "Login"), notice && /* @__PURE__ */ h(Notice, {
    message: notice,
    type: "info",
    style: "mt-2"
  }), error && /* @__PURE__ */ h(Notice, {
    message: error,
    type: "error",
    style: "mt-2"
  }), /* @__PURE__ */ h(TextInput, {
    label: "Email Address",
    placeholder: "ada.lovelace@urls.fyi",
    style: "mt-2",
    value: email,
    onChange: setEmail
  }), showCode && /* @__PURE__ */ h(TextInput, {
    label: "Login Code",
    placeholder: "12 digit code",
    style: "mt-2",
    value: code,
    onChange: setCode
  }), /* @__PURE__ */ h(Button, {
    title: showCode ? "Login" : "Request Code",
    onClick: submit,
    style: "mt-2 w-full",
    disabled: loading,
    loading
  }), /* @__PURE__ */ h("button", {
    onClick: () => setShowCode(!showCode),
    class: "w-full mt-2 text-center text-blue-500"
  }, showCode ? "I need a login code" : "I already have a login code"))));
}
render(/* @__PURE__ */ h(Login, null), document.getElementById("login"));
