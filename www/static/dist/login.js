var s,S,Q,X,H={},q=[],we=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function C(e,t){for(var n in t)e[n]=t[n];return e}function K(e){var t=e.parentNode;t&&t.removeChild(e)}function l(e,t,n){var _,i,o,c=arguments,u={};for(o in t)o=="key"?_=t[o]:o=="ref"?i=t[o]:u[o]=t[o];if(arguments.length>3)for(n=[n],o=3;o<arguments.length;o++)n.push(c[o]);if(n!=null&&(u.children=n),typeof e=="function"&&e.defaultProps!=null)for(o in e.defaultProps)u[o]===void 0&&(u[o]=e.defaultProps[o]);return P(e,u,_,i,null)}function P(e,t,n,_,i){var o={type:e,props:t,key:n,ref:_,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:i==null?++s.__v:i};return s.vnode!=null&&s.vnode(o),o}function L(e){return e.children}function E(e,t){this.props=e,this.context=t}function N(e,t){if(t==null)return e.__?N(e.__,e.__.__k.indexOf(e)+1):null;for(var n;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null)return n.__e;return typeof e.type=="function"?N(e):null}function Y(e){var t,n;if((e=e.__)!=null&&e.__c!=null){for(e.__e=e.__c.base=null,t=0;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null){e.__e=e.__c.base=n.__e;break}return Y(e)}}function Z(e){(!e.__d&&(e.__d=!0)&&S.push(e)&&!I.__r++||X!==s.debounceRendering)&&((X=s.debounceRendering)||Q)(I)}function I(){for(var e;I.__r=S.length;)e=S.sort(function(t,n){return t.__v.__b-n.__v.__b}),S=[],e.some(function(t){var n,_,i,o,c,u;t.__d&&(c=(o=(n=t).__v).__e,(u=n.__P)&&(_=[],(i=C({},o)).__v=o.__v+1,B(u,o,i,n.__n,u.ownerSVGElement!==void 0,o.__h!=null?[c]:null,_,c==null?N(o):c,o.__h),ee(_,o),o.__e!=c&&Y(o)))})}function _e(e,t,n,_,i,o,c,u,p,d){var r,v,f,a,g,h,m,y=_&&_.__k||q,w=y.length;for(n.__k=[],r=0;r<t.length;r++)if((a=n.__k[r]=(a=t[r])==null||typeof a=="boolean"?null:typeof a=="string"||typeof a=="number"||typeof a=="bigint"?P(null,a,null,null,a):Array.isArray(a)?P(L,{children:a},null,null,null):a.__b>0?P(a.type,a.props,a.key,null,a.__v):a)!=null){if(a.__=n,a.__b=n.__b+1,(f=y[r])===null||f&&a.key==f.key&&a.type===f.type)y[r]=void 0;else for(v=0;v<w;v++){if((f=y[v])&&a.key==f.key&&a.type===f.type){y[v]=void 0;break}f=null}B(e,a,f=f||H,i,o,c,u,p,d),g=a.__e,(v=a.ref)&&f.ref!=v&&(m||(m=[]),f.ref&&m.push(f.ref,null,a),m.push(v,a.__c||g,a)),g!=null?(h==null&&(h=g),typeof a.type=="function"&&a.__k!=null&&a.__k===f.__k?a.__d=p=te(a,p,e):p=ne(e,a,f,y,g,p),d||n.type!=="option"?typeof n.type=="function"&&(n.__d=p):e.value=""):p&&f.__e==p&&p.parentNode!=e&&(p=N(f))}for(n.__e=h,r=w;r--;)y[r]!=null&&(typeof n.type=="function"&&y[r].__e!=null&&y[r].__e==n.__d&&(n.__d=N(_,r+1)),re(y[r],y[r]));if(m)for(r=0;r<m.length;r++)oe(m[r],m[++r],m[++r])}function te(e,t,n){var _,i;for(_=0;_<e.__k.length;_++)(i=e.__k[_])&&(i.__=e,t=typeof i.type=="function"?te(i,t,n):ne(n,i,i,e.__k,i.__e,t));return t}function ne(e,t,n,_,i,o){var c,u,p;if(t.__d!==void 0)c=t.__d,t.__d=void 0;else if(n==null||i!=o||i.parentNode==null)e:if(o==null||o.parentNode!==e)e.appendChild(i),c=null;else{for(u=o,p=0;(u=u.nextSibling)&&p<_.length;p+=2)if(u==i)break e;e.insertBefore(i,o),c=o}return c!==void 0?c:i.nextSibling}function ke(e,t,n,_,i){var o;for(o in n)o==="children"||o==="key"||o in t||$(e,o,null,n[o],_);for(o in t)i&&typeof t[o]!="function"||o==="children"||o==="key"||o==="value"||o==="checked"||n[o]===t[o]||$(e,o,t[o],n[o],_)}function ie(e,t,n){t[0]==="-"?e.setProperty(t,n):e[t]=n==null?"":typeof n!="number"||we.test(t)?n:n+"px"}function $(e,t,n,_,i){var o;e:if(t==="style")if(typeof n=="string")e.style.cssText=n;else{if(typeof _=="string"&&(e.style.cssText=_=""),_)for(t in _)n&&t in n||ie(e.style,t,"");if(n)for(t in n)_&&n[t]===_[t]||ie(e.style,t,n[t])}else if(t[0]==="o"&&t[1]==="n")o=t!==(t=t.replace(/Capture$/,"")),t=t.toLowerCase()in e?t.toLowerCase().slice(2):t.slice(2),e.l||(e.l={}),e.l[t+o]=n,n?_||e.addEventListener(t,o?ae:le,o):e.removeEventListener(t,o?ae:le,o);else if(t!=="dangerouslySetInnerHTML"){if(i)t=t.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(t!=="href"&&t!=="list"&&t!=="form"&&t!=="tabIndex"&&t!=="download"&&t in e)try{e[t]=n==null?"":n;break e}catch(c){}typeof n=="function"||(n!=null&&(n!==!1||t[0]==="a"&&t[1]==="r")?e.setAttribute(t,n):e.removeAttribute(t))}}function le(e){this.l[e.type+!1](s.event?s.event(e):e)}function ae(e){this.l[e.type+!0](s.event?s.event(e):e)}function B(e,t,n,_,i,o,c,u,p){var d,r,v,f,a,g,h,m,y,w,A,x=t.type;if(t.constructor!==void 0)return null;n.__h!=null&&(p=n.__h,u=t.__e=n.__e,t.__h=null,o=[u]),(d=s.__b)&&d(t);try{e:if(typeof x=="function"){if(m=t.props,y=(d=x.contextType)&&_[d.__c],w=d?y?y.props.value:d.__:_,n.__c?h=(r=t.__c=n.__c).__=r.__E:("prototype"in x&&x.prototype.render?t.__c=r=new x(m,w):(t.__c=r=new E(m,w),r.constructor=x,r.render=xe),y&&y.sub(r),r.props=m,r.state||(r.state={}),r.context=w,r.__n=_,v=r.__d=!0,r.__h=[]),r.__s==null&&(r.__s=r.state),x.getDerivedStateFromProps!=null&&(r.__s==r.state&&(r.__s=C({},r.__s)),C(r.__s,x.getDerivedStateFromProps(m,r.__s))),f=r.props,a=r.state,v)x.getDerivedStateFromProps==null&&r.componentWillMount!=null&&r.componentWillMount(),r.componentDidMount!=null&&r.__h.push(r.componentDidMount);else{if(x.getDerivedStateFromProps==null&&m!==f&&r.componentWillReceiveProps!=null&&r.componentWillReceiveProps(m,w),!r.__e&&r.shouldComponentUpdate!=null&&r.shouldComponentUpdate(m,r.__s,w)===!1||t.__v===n.__v){r.props=m,r.state=r.__s,t.__v!==n.__v&&(r.__d=!1),r.__v=t,t.__e=n.__e,t.__k=n.__k,t.__k.forEach(function(T){T&&(T.__=t)}),r.__h.length&&c.push(r);break e}r.componentWillUpdate!=null&&r.componentWillUpdate(m,r.__s,w),r.componentDidUpdate!=null&&r.__h.push(function(){r.componentDidUpdate(f,a,g)})}r.context=w,r.props=m,r.state=r.__s,(d=s.__r)&&d(t),r.__d=!1,r.__v=t,r.__P=e,d=r.render(r.props,r.state,r.context),r.state=r.__s,r.getChildContext!=null&&(_=C(C({},_),r.getChildContext())),v||r.getSnapshotBeforeUpdate==null||(g=r.getSnapshotBeforeUpdate(f,a)),A=d!=null&&d.type===L&&d.key==null?d.props.children:d,_e(e,Array.isArray(A)?A:[A],t,n,_,i,o,c,u,p),r.base=t.__e,t.__h=null,r.__h.length&&c.push(r),h&&(r.__E=r.__=null),r.__e=!1}else o==null&&t.__v===n.__v?(t.__k=n.__k,t.__e=n.__e):t.__e=be(n.__e,t,n,_,i,o,c,p);(d=s.diffed)&&d(t)}catch(T){t.__v=null,(p||o!=null)&&(t.__e=u,t.__h=!!p,o[o.indexOf(u)]=null),s.__e(T,t,n)}}function ee(e,t){s.__c&&s.__c(t,e),e.some(function(n){try{e=n.__h,n.__h=[],e.some(function(_){_.call(n)})}catch(_){s.__e(_,n.__v)}})}function be(e,t,n,_,i,o,c,u){var p,d,r,v,f=n.props,a=t.props,g=t.type,h=0;if(g==="svg"&&(i=!0),o!=null){for(;h<o.length;h++)if((p=o[h])&&(p===e||(g?p.localName==g:p.nodeType==3))){e=p,o[h]=null;break}}if(e==null){if(g===null)return document.createTextNode(a);e=i?document.createElementNS("http://www.w3.org/2000/svg",g):document.createElement(g,a.is&&a),o=null,u=!1}if(g===null)f===a||u&&e.data===a||(e.data=a);else{if(o=o&&q.slice.call(e.childNodes),d=(f=n.props||H).dangerouslySetInnerHTML,r=a.dangerouslySetInnerHTML,!u){if(o!=null)for(f={},v=0;v<e.attributes.length;v++)f[e.attributes[v].name]=e.attributes[v].value;(r||d)&&(r&&(d&&r.__html==d.__html||r.__html===e.innerHTML)||(e.innerHTML=r&&r.__html||""))}if(ke(e,a,f,i,u),r)t.__k=[];else if(h=t.props.children,_e(e,Array.isArray(h)?h:[h],t,n,_,i&&g!=="foreignObject",o,c,e.firstChild,u),o!=null)for(h=o.length;h--;)o[h]!=null&&K(o[h]);u||("value"in a&&(h=a.value)!==void 0&&(h!==e.value||g==="progress"&&!h)&&$(e,"value",h,f.value,!1),"checked"in a&&(h=a.checked)!==void 0&&h!==e.checked&&$(e,"checked",h,f.checked,!1))}return e}function oe(e,t,n){try{typeof e=="function"?e(t):e.current=t}catch(_){s.__e(_,n)}}function re(e,t,n){var _,i,o;if(s.unmount&&s.unmount(e),(_=e.ref)&&(_.current&&_.current!==e.__e||oe(_,null,t)),n||typeof e.type=="function"||(n=(i=e.__e)!=null),e.__e=e.__d=void 0,(_=e.__c)!=null){if(_.componentWillUnmount)try{_.componentWillUnmount()}catch(c){s.__e(c,t)}_.base=_.__P=null}if(_=e.__k)for(o=0;o<_.length;o++)_[o]&&re(_[o],t,n);i!=null&&K(i)}function xe(e,t,n){return this.constructor(e,n)}function R(e,t,n){var _,i,o;s.__&&s.__(e,t),i=(_=typeof n=="function")?null:n&&n.__k||t.__k,o=[],B(t,e=(!_&&n||t).__k=l(L,null,[e]),i||H,H,t.ownerSVGElement!==void 0,!_&&n?[n]:i?null:t.firstChild?q.slice.call(t.childNodes):null,o,!_&&n?n:i?i.__e:t.firstChild,_),ee(o,e)}s={__e:function(e,t){for(var n,_,i;t=t.__;)if((n=t.__c)&&!n.__)try{if((_=n.constructor)&&_.getDerivedStateFromError!=null&&(n.setState(_.getDerivedStateFromError(e)),i=n.__d),n.componentDidCatch!=null&&(n.componentDidCatch(e),i=n.__d),i)return n.__E=n}catch(o){e=o}throw e},__v:0},E.prototype.setState=function(e,t){var n;n=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=C({},this.state),typeof e=="function"&&(e=e(C({},n),this.props)),e&&C(n,e),e!=null&&this.__v&&(t&&this.__h.push(t),Z(this))},E.prototype.forceUpdate=function(e){this.__v&&(this.__e=!0,e&&this.__h.push(e),Z(this))},E.prototype.render=L,S=[],Q=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,I.__r=0;var se,k,ce,U=0,O=[],ue=s.__b,fe=s.__r,pe=s.diffed,de=s.__c,he=s.unmount;function Ce(e,t){s.__h&&s.__h(k,e,U||t),U=0;var n=k.__H||(k.__H={__:[],__h:[]});return e>=n.__.length&&n.__.push({}),n.__[e]}function b(e){return U=1,Ee(me,e)}function Ee(e,t,n){var _=Ce(se++,2);return _.t=e,_.__c||(_.__=[n?n(t):me(void 0,t),function(i){var o=_.t(_.__[0],i);_.__[0]!==o&&(_.__=[o,_.__[1]],_.__c.setState({}))}],_.__c=k),_.__}function Se(){O.forEach(function(e){if(e.__P)try{e.__H.__h.forEach(F),e.__H.__h.forEach(W),e.__H.__h=[]}catch(t){e.__H.__h=[],s.__e(t,e.__v)}}),O=[]}s.__b=function(e){k=null,ue&&ue(e)},s.__r=function(e){fe&&fe(e),se=0;var t=(k=e.__c).__H;t&&(t.__h.forEach(F),t.__h.forEach(W),t.__h=[])},s.diffed=function(e){pe&&pe(e);var t=e.__c;t&&t.__H&&t.__H.__h.length&&(O.push(t)!==1&&ce===s.requestAnimationFrame||((ce=s.requestAnimationFrame)||function(n){var _,i=function(){clearTimeout(o),ge&&cancelAnimationFrame(_),setTimeout(n)},o=setTimeout(i,100);ge&&(_=requestAnimationFrame(i))})(Se)),k=void 0},s.__c=function(e,t){t.some(function(n){try{n.__h.forEach(F),n.__h=n.__h.filter(function(_){return!_.__||W(_)})}catch(_){t.some(function(i){i.__h&&(i.__h=[])}),t=[],s.__e(_,n.__v)}}),de&&de(e,t)},s.unmount=function(e){he&&he(e);var t=e.__c;if(t&&t.__H)try{t.__H.__.forEach(F)}catch(n){s.__e(n,t.__v)}};var ge=typeof requestAnimationFrame=="function";function F(e){var t=k;typeof e.__c=="function"&&e.__c(),k=t}function W(e){var t=k;e.__c=e.__(),k=t}function me(e,t){return typeof t=="function"?t(e):t}async function Ne(e,t){let n=window.__xsrf_token;return await(await fetch("/graphql",{method:"POST",credentials:"include",headers:{"Content-Type":"application/json","X-XSRF-Token":n},body:JSON.stringify({query:e,variables:t})})).json()}function z([e]){return e}function V(e,{onCommit:t,onError:n}){let[_,i]=b(!1),[o,c]=b(void 0);if(o!==void 0)throw o;return{commit:p=>{(async()=>{i(!0);let{data:d,errors:r}=await Ne(e,p);i(!1),r!=null?typeof n=="function"?n(r):c(error):t(d)})()},inFlight:_}}function G({size:e="small"}){return l("svg",{xmlns:"http://www.w3.org/2000/svg",class:e==="large"?"h-8 w-8":"h-4 w-4",viewBox:"0 0 24 24",stroke:"none",fill:"currentColor"},l("rect",{x:"10",y:"0",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 6.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.125s",repeatCount:"indefinite"})),l("rect",{x:"16",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.25s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 17.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.375s",repeatCount:"indefinite"})),l("rect",{x:"10",y:"16",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.5s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 17.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.625s",repeatCount:"indefinite"})),l("rect",{x:"0",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.75s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 6.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.875s",repeatCount:"indefinite"})))}function j({title:e,onClick:t,disabled:n=!1,loading:_=!1,type:i="default",style:o}){let c=`h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${o!=null?o:""}`;switch(i){case"flat":c+=" bg-white text-black",n||(c+=" hover:bg-gray-100");break;case"default":default:c+=" bg-blue-500 text-white",n||(c+=" hover:bg-blue-400")}return n===!0&&(c+=" opacity-80 cursor-not-allowed"),l("button",{class:c,onClick:n===!0?void 0:t},l("div",null,e),_===!0&&l(G,null))}function je(){window.location.href=window.location.href}function Ae({error:e}){return l("div",{class:"w-full flex justify-center p-8"},l("div",{class:"w-full max-w-3xl bg-red-500 rounded-lg p-4"},l("h1",{class:"text-white text-xl font-semibold"},"Error"),l("p",{class:"text-red-100"},"Sorry, something went wrong."),l("pre",{class:"text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap"},JSON.stringify(e)),l(j,{title:"Reload Page",onClick:je,type:"flat",style:"w-full mt-2"})))}var J=class extends E{constructor(){super();this.state={error:!1}}componentDidCatch(t){console.error(t),this.setState({error:t})}render({children:t},{error:n}){return n!==!1?l(Ae,{error:n}):t}},ve=J;function M({value:e,onChange:t,label:n,placeholder:_,style:i}){let[o]=b(`input-id-${Math.floor(Math.random()*1e4)}`);return l("div",{class:`w-full ${i!=null?i:""}`},l("label",{class:"text-gray-500 italic",for:o},n),l("input",{class:"w-full bg-gray-200 p-2 text-md rounded-md",id:o,type:"text",value:e,onChange:c=>typeof t=="function"&&t(c.target.value),placeholder:_}))}var Te=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"})),ye=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}));function D({type:e="info",message:t,style:n}){let _=`flex align-start p-3 rounded-md block font-semibold ${n!=null?n:""}`,i;switch(e){case"error":_+=" bg-red-200 text-red-800",i=ye;break;case"warning":_+=" bg-yellow-200 text-yellow-800",i=ye;break;case"info":default:_+=" bg-blue-200 text-blue-800",i=Te}return l("div",{class:_},l("div",{class:"mr-2"},i),l("p",{class:"leading-snug"},t))}function He(){let[e,t]=b(""),[n,_]=b(""),[i,o]=b(!1),[c,u]=b(null),[p,d]=b(null),r=V(z`
    mutation RequestLogin($email: String!) {
      requestLogin(email: $email) {
        ok
      }
    }
  `,{onCommit:()=>{d(`A login code was sent to ${e}.`),u(null),o(!0)},onError:m=>{u(`Could not request code: ${m[0].message}`)}}),v=()=>r.commit({email:e}),f=V(z`
    mutation Login($email: String!, $code: String!) {
      login(email: $email, token: $code)
    }
  `,{onCommit:({login:m})=>{document.cookie=`${window.__auth_cookie}=${m};path=/;max-age=604800`,window.location.href="/"},onError:m=>{u(`Failed to log in: ${m[0].message}`)}}),a=()=>f.commit({email:e,code:n}),g=r.inFlight||f.inFlight;return l(ve,null,l("div",{class:"w-full flex justify-center p-8"},l("div",{class:"w-full max-w-md bg-white shadow rounded-lg p-4"},l("h1",{class:"text-2xl font-semibold"},"Login"),p&&l(D,{message:p,type:"info",style:"mt-2"}),c&&l(D,{message:c,type:"error",style:"mt-2"}),l(M,{label:"Email Address",placeholder:"ada.lovelace@urls.fyi",style:"mt-2",value:e,onChange:t}),i&&l(M,{label:"Login Code",placeholder:"12 digit code",style:"mt-2",value:n,onChange:_}),l(j,{title:i?"Login":"Request Code",onClick:i?a:v,style:"mt-2 w-full",disabled:g,loading:g}),l("button",{onClick:()=>o(!i),class:"w-full mt-2 text-center text-blue-500"},i?"I need a login code":"I already have a login code"))))}R(l(He,null),document.getElementById("login"));
//# sourceMappingURL=login.js.map
