var s,S,G,J,A={},L=[],ke=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function C(e,t){for(var n in t)e[n]=t[n];return e}function Q(e){var t=e.parentNode;t&&t.removeChild(e)}function a(e,t,n){var _,i,o,c=arguments,u={};for(o in t)o=="key"?_=t[o]:o=="ref"?i=t[o]:u[o]=t[o];if(arguments.length>3)for(n=[n],o=3;o<arguments.length;o++)n.push(c[o]);if(n!=null&&(u.children=n),typeof e=="function"&&e.defaultProps!=null)for(o in e.defaultProps)u[o]===void 0&&(u[o]=e.defaultProps[o]);return D(e,u,_,i,null)}function D(e,t,n,_,i){var o={type:e,props:t,key:n,ref:_,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:i==null?++s.__v:i};return s.vnode!=null&&s.vnode(o),o}function N(e){return e.children}function E(e,t){this.props=e,this.context=t}function j(e,t){if(t==null)return e.__?j(e.__,e.__.__k.indexOf(e)+1):null;for(var n;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null)return n.__e;return typeof e.type=="function"?j(e):null}function X(e){var t,n;if((e=e.__)!=null&&e.__c!=null){for(e.__e=e.__c.base=null,t=0;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null){e.__e=e.__c.base=n.__e;break}return X(e)}}function Y(e){(!e.__d&&(e.__d=!0)&&S.push(e)&&!M.__r++||J!==s.debounceRendering)&&((J=s.debounceRendering)||G)(M)}function M(){for(var e;M.__r=S.length;)e=S.sort(function(t,n){return t.__v.__b-n.__v.__b}),S=[],e.some(function(t){var n,_,i,o,c,u;t.__d&&(c=(o=(n=t).__v).__e,(u=n.__P)&&(_=[],(i=C({},o)).__v=o.__v+1,R(u,o,i,n.__n,u.ownerSVGElement!==void 0,o.__h!=null?[c]:null,_,c==null?j(o):c,o.__h),K(_,o),o.__e!=c&&X(o)))})}function oe(e,t,n,_,i,o,c,u,p,f){var r,m,d,l,v,h,g,y=_&&_.__k||L,k=y.length;for(n.__k=[],r=0;r<t.length;r++)if((l=n.__k[r]=(l=t[r])==null||typeof l=="boolean"?null:typeof l=="string"||typeof l=="number"||typeof l=="bigint"?D(null,l,null,null,l):Array.isArray(l)?D(N,{children:l},null,null,null):l.__b>0?D(l.type,l.props,l.key,null,l.__v):l)!=null){if(l.__=n,l.__b=n.__b+1,(d=y[r])===null||d&&l.key==d.key&&l.type===d.type)y[r]=void 0;else for(m=0;m<k;m++){if((d=y[m])&&l.key==d.key&&l.type===d.type){y[m]=void 0;break}d=null}R(e,l,d=d||A,i,o,c,u,p,f),v=l.__e,(m=l.ref)&&d.ref!=m&&(g||(g=[]),d.ref&&g.push(d.ref,null,l),g.push(m,l.__c||v,l)),v!=null?(h==null&&(h=v),typeof l.type=="function"&&l.__k!=null&&l.__k===d.__k?l.__d=p=Z(l,p,e):p=ee(e,l,d,y,v,p),f||n.type!=="option"?typeof n.type=="function"&&(n.__d=p):e.value=""):p&&d.__e==p&&p.parentNode!=e&&(p=j(d))}for(n.__e=h,r=k;r--;)y[r]!=null&&(typeof n.type=="function"&&y[r].__e!=null&&y[r].__e==n.__d&&(n.__d=j(_,r+1)),ne(y[r],y[r]));if(g)for(r=0;r<g.length;r++)te(g[r],g[++r],g[++r])}function Z(e,t,n){var _,i;for(_=0;_<e.__k.length;_++)(i=e.__k[_])&&(i.__=e,t=typeof i.type=="function"?Z(i,t,n):ee(n,i,i,e.__k,i.__e,t));return t}function ee(e,t,n,_,i,o){var c,u,p;if(t.__d!==void 0)c=t.__d,t.__d=void 0;else if(n==null||i!=o||i.parentNode==null)e:if(o==null||o.parentNode!==e)e.appendChild(i),c=null;else{for(u=o,p=0;(u=u.nextSibling)&&p<_.length;p+=2)if(u==i)break e;e.insertBefore(i,o),c=o}return c!==void 0?c:i.nextSibling}function be(e,t,n,_,i){var o;for(o in n)o==="children"||o==="key"||o in t||F(e,o,null,n[o],_);for(o in t)i&&typeof t[o]!="function"||o==="children"||o==="key"||o==="value"||o==="checked"||n[o]===t[o]||F(e,o,t[o],n[o],_)}function re(e,t,n){t[0]==="-"?e.setProperty(t,n):e[t]=n==null?"":typeof n!="number"||ke.test(t)?n:n+"px"}function F(e,t,n,_,i){var o;e:if(t==="style")if(typeof n=="string")e.style.cssText=n;else{if(typeof _=="string"&&(e.style.cssText=_=""),_)for(t in _)n&&t in n||re(e.style,t,"");if(n)for(t in n)_&&n[t]===_[t]||re(e.style,t,n[t])}else if(t[0]==="o"&&t[1]==="n")o=t!==(t=t.replace(/Capture$/,"")),t=t.toLowerCase()in e?t.toLowerCase().slice(2):t.slice(2),e.l||(e.l={}),e.l[t+o]=n,n?_||e.addEventListener(t,o?ie:_e,o):e.removeEventListener(t,o?ie:_e,o);else if(t!=="dangerouslySetInnerHTML"){if(i)t=t.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(t!=="href"&&t!=="list"&&t!=="form"&&t!=="tabIndex"&&t!=="download"&&t in e)try{e[t]=n==null?"":n;break e}catch(c){}typeof n=="function"||(n!=null&&(n!==!1||t[0]==="a"&&t[1]==="r")?e.setAttribute(t,n):e.removeAttribute(t))}}function _e(e){this.l[e.type+!1](s.event?s.event(e):e)}function ie(e){this.l[e.type+!0](s.event?s.event(e):e)}function R(e,t,n,_,i,o,c,u,p){var f,r,m,d,l,v,h,g,y,k,P,w=t.type;if(t.constructor!==void 0)return null;n.__h!=null&&(p=n.__h,u=t.__e=n.__e,t.__h=null,o=[u]),(f=s.__b)&&f(t);try{e:if(typeof w=="function"){if(g=t.props,y=(f=w.contextType)&&_[f.__c],k=f?y?y.props.value:f.__:_,n.__c?h=(r=t.__c=n.__c).__=r.__E:("prototype"in w&&w.prototype.render?t.__c=r=new w(g,k):(t.__c=r=new E(g,k),r.constructor=w,r.render=xe),y&&y.sub(r),r.props=g,r.state||(r.state={}),r.context=k,r.__n=_,m=r.__d=!0,r.__h=[]),r.__s==null&&(r.__s=r.state),w.getDerivedStateFromProps!=null&&(r.__s==r.state&&(r.__s=C({},r.__s)),C(r.__s,w.getDerivedStateFromProps(g,r.__s))),d=r.props,l=r.state,m)w.getDerivedStateFromProps==null&&r.componentWillMount!=null&&r.componentWillMount(),r.componentDidMount!=null&&r.__h.push(r.componentDidMount);else{if(w.getDerivedStateFromProps==null&&g!==d&&r.componentWillReceiveProps!=null&&r.componentWillReceiveProps(g,k),!r.__e&&r.shouldComponentUpdate!=null&&r.shouldComponentUpdate(g,r.__s,k)===!1||t.__v===n.__v){r.props=g,r.state=r.__s,t.__v!==n.__v&&(r.__d=!1),r.__v=t,t.__e=n.__e,t.__k=n.__k,t.__k.forEach(function(H){H&&(H.__=t)}),r.__h.length&&c.push(r);break e}r.componentWillUpdate!=null&&r.componentWillUpdate(g,r.__s,k),r.componentDidUpdate!=null&&r.__h.push(function(){r.componentDidUpdate(d,l,v)})}r.context=k,r.props=g,r.state=r.__s,(f=s.__r)&&f(t),r.__d=!1,r.__v=t,r.__P=e,f=r.render(r.props,r.state,r.context),r.state=r.__s,r.getChildContext!=null&&(_=C(C({},_),r.getChildContext())),m||r.getSnapshotBeforeUpdate==null||(v=r.getSnapshotBeforeUpdate(d,l)),P=f!=null&&f.type===N&&f.key==null?f.props.children:f,oe(e,Array.isArray(P)?P:[P],t,n,_,i,o,c,u,p),r.base=t.__e,t.__h=null,r.__h.length&&c.push(r),h&&(r.__E=r.__=null),r.__e=!1}else o==null&&t.__v===n.__v?(t.__k=n.__k,t.__e=n.__e):t.__e=we(n.__e,t,n,_,i,o,c,p);(f=s.diffed)&&f(t)}catch(H){t.__v=null,(p||o!=null)&&(t.__e=u,t.__h=!!p,o[o.indexOf(u)]=null),s.__e(H,t,n)}}function K(e,t){s.__c&&s.__c(t,e),e.some(function(n){try{e=n.__h,n.__h=[],e.some(function(_){_.call(n)})}catch(_){s.__e(_,n.__v)}})}function we(e,t,n,_,i,o,c,u){var p,f,r,m,d=n.props,l=t.props,v=t.type,h=0;if(v==="svg"&&(i=!0),o!=null){for(;h<o.length;h++)if((p=o[h])&&(p===e||(v?p.localName==v:p.nodeType==3))){e=p,o[h]=null;break}}if(e==null){if(v===null)return document.createTextNode(l);e=i?document.createElementNS("http://www.w3.org/2000/svg",v):document.createElement(v,l.is&&l),o=null,u=!1}if(v===null)d===l||u&&e.data===l||(e.data=l);else{if(o=o&&L.slice.call(e.childNodes),f=(d=n.props||A).dangerouslySetInnerHTML,r=l.dangerouslySetInnerHTML,!u){if(o!=null)for(d={},m=0;m<e.attributes.length;m++)d[e.attributes[m].name]=e.attributes[m].value;(r||f)&&(r&&(f&&r.__html==f.__html||r.__html===e.innerHTML)||(e.innerHTML=r&&r.__html||""))}if(be(e,l,d,i,u),r)t.__k=[];else if(h=t.props.children,oe(e,Array.isArray(h)?h:[h],t,n,_,i&&v!=="foreignObject",o,c,e.firstChild,u),o!=null)for(h=o.length;h--;)o[h]!=null&&Q(o[h]);u||("value"in l&&(h=l.value)!==void 0&&(h!==e.value||v==="progress"&&!h)&&F(e,"value",h,d.value,!1),"checked"in l&&(h=l.checked)!==void 0&&h!==e.checked&&F(e,"checked",h,d.checked,!1))}return e}function te(e,t,n){try{typeof e=="function"?e(t):e.current=t}catch(_){s.__e(_,n)}}function ne(e,t,n){var _,i,o;if(s.unmount&&s.unmount(e),(_=e.ref)&&(_.current&&_.current!==e.__e||te(_,null,t)),n||typeof e.type=="function"||(n=(i=e.__e)!=null),e.__e=e.__d=void 0,(_=e.__c)!=null){if(_.componentWillUnmount)try{_.componentWillUnmount()}catch(c){s.__e(c,t)}_.base=_.__P=null}if(_=e.__k)for(o=0;o<_.length;o++)_[o]&&ne(_[o],t,n);i!=null&&Q(i)}function xe(e,t,n){return this.constructor(e,n)}function B(e,t,n){var _,i,o;s.__&&s.__(e,t),i=(_=typeof n=="function")?null:n&&n.__k||t.__k,o=[],R(t,e=(!_&&n||t).__k=a(N,null,[e]),i||A,A,t.ownerSVGElement!==void 0,!_&&n?[n]:i?null:t.firstChild?L.slice.call(t.childNodes):null,o,!_&&n?n:i?i.__e:t.firstChild,_),K(o,e)}s={__e:function(e,t){for(var n,_,i;t=t.__;)if((n=t.__c)&&!n.__)try{if((_=n.constructor)&&_.getDerivedStateFromError!=null&&(n.setState(_.getDerivedStateFromError(e)),i=n.__d),n.componentDidCatch!=null&&(n.componentDidCatch(e),i=n.__d),i)return n.__E=n}catch(o){e=o}throw e},__v:0},E.prototype.setState=function(e,t){var n;n=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=C({},this.state),typeof e=="function"&&(e=e(C({},n),this.props)),e&&C(n,e),e!=null&&this.__v&&(t&&this.__h.push(t),Y(this))},E.prototype.forceUpdate=function(e){this.__v&&(this.__e=!0,e&&this.__h.push(e),Y(this))},E.prototype.render=N,S=[],G=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,M.__r=0;var ae,b,le,U=0,O=[],se=s.__b,ce=s.__r,ue=s.diffed,fe=s.__c,pe=s.unmount;function Ce(e,t){s.__h&&s.__h(b,e,U||t),U=0;var n=b.__H||(b.__H={__:[],__h:[]});return e>=n.__.length&&n.__.push({}),n.__[e]}function x(e){return U=1,Ee(de,e)}function Ee(e,t,n){var _=Ce(ae++,2);return _.t=e,_.__c||(_.__=[n?n(t):de(void 0,t),function(i){var o=_.t(_.__[0],i);_.__[0]!==o&&(_.__=[o,_.__[1]],_.__c.setState({}))}],_.__c=b),_.__}function Se(){O.forEach(function(e){if(e.__P)try{e.__H.__h.forEach($),e.__H.__h.forEach(W),e.__H.__h=[]}catch(t){e.__H.__h=[],s.__e(t,e.__v)}}),O=[]}s.__b=function(e){b=null,se&&se(e)},s.__r=function(e){ce&&ce(e),ae=0;var t=(b=e.__c).__H;t&&(t.__h.forEach($),t.__h.forEach(W),t.__h=[])},s.diffed=function(e){ue&&ue(e);var t=e.__c;t&&t.__H&&t.__H.__h.length&&(O.push(t)!==1&&le===s.requestAnimationFrame||((le=s.requestAnimationFrame)||function(n){var _,i=function(){clearTimeout(o),he&&cancelAnimationFrame(_),setTimeout(n)},o=setTimeout(i,100);he&&(_=requestAnimationFrame(i))})(Se)),b=void 0},s.__c=function(e,t){t.some(function(n){try{n.__h.forEach($),n.__h=n.__h.filter(function(_){return!_.__||W(_)})}catch(_){t.some(function(i){i.__h&&(i.__h=[])}),t=[],s.__e(_,n.__v)}}),fe&&fe(e,t)},s.unmount=function(e){pe&&pe(e);var t=e.__c;if(t&&t.__H)try{t.__H.__.forEach($)}catch(n){s.__e(n,t.__v)}};var he=typeof requestAnimationFrame=="function";function $(e){var t=b;typeof e.__c=="function"&&e.__c(),b=t}function W(e){var t=b;e.__c=e.__(),b=t}function de(e,t){return typeof t=="function"?t(e):t}async function Ne(e,t){let n=window.__xsrf_token;return await(await fetch("/graphql",{method:"POST",credentials:"include",headers:{"Content-Type":"application/json","X-XSRF-Token":n},body:JSON.stringify({query:e,variables:t})})).json()}function me([e]){return e}function ge(e,{onCommit:t,onError:n}){let[_,i]=x(!1),[o,c]=x(void 0);if(o!==void 0)throw o;return{commit:p=>{i(!0),Ne(e,p).then(({data:f,errors:r})=>{i(!1),r!=null?typeof n=="function"?n(r):c(r):t(f)}).catch(f=>c(f))},inFlight:_}}function q({size:e="small",style:t=""}){return a("svg",{xmlns:"http://www.w3.org/2000/svg",class:`${e==="large"?"h-8 w-8":"h-4 w-4"} ${t!=null?t:""}`,viewBox:"0 0 24 24",stroke:"none",fill:"currentColor"},a("rect",{x:"10",y:"0",width:"4",height:"8",rx:"2",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0s",repeatCount:"indefinite"})),a("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 6.5) rotate(45)",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.125s",repeatCount:"indefinite"})),a("rect",{x:"16",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.25s",repeatCount:"indefinite"})),a("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 17.5) rotate(-45)",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.375s",repeatCount:"indefinite"})),a("rect",{x:"10",y:"16",width:"4",height:"8",rx:"2",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.5s",repeatCount:"indefinite"})),a("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 17.5) rotate(45)",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.625s",repeatCount:"indefinite"})),a("rect",{x:"0",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.75s",repeatCount:"indefinite"})),a("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 6.5) rotate(-45)",opacity:"0"},a("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.875s",repeatCount:"indefinite"})))}function T({title:e,onClick:t,disabled:n=!1,loading:_=!1,type:i="default",style:o}){let c=`h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${o!=null?o:""}`;switch(i){case"flat":c+=" bg-white text-black",n||(c+=" hover:bg-gray-200");break;case"default":default:c+=" bg-blue-500 text-white",n||(c+=" hover:bg-blue-400")}return n===!0&&(c+=" opacity-80 cursor-not-allowed"),a("button",{class:c,onClick:n===!0?void 0:t},a("div",null,e),_===!0&&a(q,null))}function je(){window.location.href=window.location.href}function Te(e){let t={};for(let n of Object.getOwnPropertyNames(e))t[n]=e[n];return JSON.stringify(t)}function Ie({error:e}){return a("div",{class:"w-full flex justify-center p-8"},a("div",{class:"w-full max-w-3xl bg-red-500 rounded-lg p-4"},a("h1",{class:"text-white text-xl font-semibold"},"Error"),a("p",{class:"text-red-100"},"Sorry, something went wrong."),a("pre",{class:"text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap break-all"},Te(e)),a(T,{title:"Reload Page",onClick:je,type:"flat",style:"w-full mt-2"})))}var z=class extends E{constructor(){super();this.state={error:!1}}componentDidCatch(t){console.error(t),this.setState({error:t})}render({children:t},{error:n}){return n!==!1?a(Ie,{error:n}):t}},ve=z;function I({value:e,onChange:t,label:n,placeholder:_,style:i}){let[o]=x(`input-id-${Math.floor(Math.random()*1e4)}`);return a("div",{class:`w-full ${i!=null?i:""}`},a("label",{class:"text-gray-500 italic",for:o},n),a("input",{class:"w-full p-2 text-md rounded-md bg-gray-200 dark:bg-gray-600 text-black dark:text-white",id:o,type:"text",value:e,onInput:c=>typeof t=="function"&&t(c.target.value),placeholder:_}))}var Pe=a("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},a("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"})),ye=a("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},a("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}));function V({type:e="info",message:t,style:n}){let _=`flex align-start p-3 rounded-md block font-semibold ${n!=null?n:""}`,i;switch(e){case"error":_+=" bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",i=ye;break;case"warning":_+=" bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",i=ye;break;case"info":default:_+=" bg-blue-200 text-blue-800 dark:bg-blue-800 dark:text-blue-200",i=Pe}return a("div",{class:_},a("div",{class:"mr-2"},i),a("p",{class:"leading-snug"},t))}function He(){let[e,t]=x(""),[n,_]=x(""),[i,o]=x(""),[c,u]=x(null),{commit:p,inFlight:f}=ge(me`
    mutation RegisterMutation($input: NewUserInput!, $code: String!) {
      registerUser(input: $input, token: $code) {
        id
      }
    }
  `,{onCommit:()=>window.location.href="/login",onError:([{message:m}])=>u(`Failed to register: ${m}`)}),r=m=>{m.preventDefault(),p({input:{name:e,email:n},code:i})};return a("div",{class:"w-full flex justify-center p-8"},a("form",{class:"w-full max-w-md bg-white dark:bg-gray-800 shadow rounded-lg p-4",onSubmit:r},a("h1",{class:"text-2xl font-semibold"},"Register"),c&&a(V,{message:c,type:"error",style:"mt-2"}),a(I,{label:"Your name",placeholder:"Ada Lovelace",style:"mt-2",value:e,onChange:t}),a(I,{label:"Email Address",placeholder:"ada.lovelace@urls.fyi",style:"mt-2",value:n,onChange:_}),a(I,{label:"Invitation Code",placeholder:"Your invitation code",style:"mt-2",value:i,onChange:o}),a(T,{title:"Register",onClick:r,style:"mt-2 w-full",disabled:f,loading:f})))}B(a(ve,null,a(He,null)),document.getElementById("register"));
//# sourceMappingURL=register.js.map