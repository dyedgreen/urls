var f,P,ne,oe,$={},z=[],Se=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function E(e,t){for(var n in t)e[n]=t[n];return e}function re(e){var t=e.parentNode;t&&t.removeChild(e)}function l(e,t,n){var r,a,o,c=arguments,u={};for(o in t)o=="key"?r=t[o]:o=="ref"?a=t[o]:u[o]=t[o];if(arguments.length>3)for(n=[n],o=3;o<arguments.length;o++)n.push(c[o]);if(n!=null&&(u.children=n),typeof e=="function"&&e.defaultProps!=null)for(o in e.defaultProps)u[o]===void 0&&(u[o]=e.defaultProps[o]);return O(e,u,r,a,null)}function O(e,t,n,r,a){var o={type:e,props:t,key:n,ref:r,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:a==null?++f.__v:a};return f.vnode!=null&&f.vnode(o),o}function I(e){return e.children}function S(e,t){this.props=e,this.context=t}function H(e,t){if(t==null)return e.__?H(e.__,e.__.__k.indexOf(e)+1):null;for(var n;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null)return n.__e;return typeof e.type=="function"?H(e):null}function ie(e){var t,n;if((e=e.__)!=null&&e.__c!=null){for(e.__e=e.__c.base=null,t=0;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null){e.__e=e.__c.base=n.__e;break}return ie(e)}}function le(e){(!e.__d&&(e.__d=!0)&&P.push(e)&&!q.__r++||oe!==f.debounceRendering)&&((oe=f.debounceRendering)||ne)(q)}function q(){for(var e;q.__r=P.length;)e=P.sort(function(t,n){return t.__v.__b-n.__v.__b}),P=[],e.some(function(t){var n,r,a,o,c,u;t.__d&&(c=(o=(n=t).__v).__e,(u=n.__P)&&(r=[],(a=E({},o)).__v=o.__v+1,Q(u,o,a,n.__n,u.ownerSVGElement!==void 0,o.__h!=null?[c]:null,r,c==null?H(o):c,o.__h),ae(r,o),o.__e!=c&&ie(o)))})}function fe(e,t,n,r,a,o,c,u,m,p){var i,d,_,s,y,h,v,k=r&&r.__k||z,b=k.length;for(n.__k=[],i=0;i<t.length;i++)if((s=n.__k[i]=(s=t[i])==null||typeof s=="boolean"?null:typeof s=="string"||typeof s=="number"||typeof s=="bigint"?O(null,s,null,null,s):Array.isArray(s)?O(I,{children:s},null,null,null):s.__b>0?O(s.type,s.props,s.key,null,s.__v):s)!=null){if(s.__=n,s.__b=n.__b+1,(_=k[i])===null||_&&s.key==_.key&&s.type===_.type)k[i]=void 0;else for(d=0;d<b;d++){if((_=k[d])&&s.key==_.key&&s.type===_.type){k[d]=void 0;break}_=null}Q(e,s,_=_||$,a,o,c,u,m,p),y=s.__e,(d=s.ref)&&_.ref!=d&&(v||(v=[]),_.ref&&v.push(_.ref,null,s),v.push(d,s.__c||y,s)),y!=null?(h==null&&(h=y),typeof s.type=="function"&&s.__k!=null&&s.__k===_.__k?s.__d=m=se(s,m,e):m=_e(e,s,_,k,y,m),p||n.type!=="option"?typeof n.type=="function"&&(n.__d=m):e.value=""):m&&_.__e==m&&m.parentNode!=e&&(m=H(_))}for(n.__e=h,i=b;i--;)k[i]!=null&&(typeof n.type=="function"&&k[i].__e!=null&&k[i].__e==n.__d&&(n.__d=H(r,i+1)),ue(k[i],k[i]));if(v)for(i=0;i<v.length;i++)ce(v[i],v[++i],v[++i])}function se(e,t,n){var r,a;for(r=0;r<e.__k.length;r++)(a=e.__k[r])&&(a.__=e,t=typeof a.type=="function"?se(a,t,n):_e(n,a,a,e.__k,a.__e,t));return t}function _e(e,t,n,r,a,o){var c,u,m;if(t.__d!==void 0)c=t.__d,t.__d=void 0;else if(n==null||a!=o||a.parentNode==null)e:if(o==null||o.parentNode!==e)e.appendChild(a),c=null;else{for(u=o,m=0;(u=u.nextSibling)&&m<r.length;m+=2)if(u==a)break e;e.insertBefore(a,o),c=o}return c!==void 0?c:a.nextSibling}function Me(e,t,n,r,a){var o;for(o in n)o==="children"||o==="key"||o in t||B(e,o,null,n[o],r);for(o in t)a&&typeof t[o]!="function"||o==="children"||o==="key"||o==="value"||o==="checked"||n[o]===t[o]||B(e,o,t[o],n[o],r)}function pe(e,t,n){t[0]==="-"?e.setProperty(t,n):e[t]=n==null?"":typeof n!="number"||Se.test(t)?n:n+"px"}function B(e,t,n,r,a){var o;e:if(t==="style")if(typeof n=="string")e.style.cssText=n;else{if(typeof r=="string"&&(e.style.cssText=r=""),r)for(t in r)n&&t in n||pe(e.style,t,"");if(n)for(t in n)r&&n[t]===r[t]||pe(e.style,t,n[t])}else if(t[0]==="o"&&t[1]==="n")o=t!==(t=t.replace(/Capture$/,"")),t=t.toLowerCase()in e?t.toLowerCase().slice(2):t.slice(2),e.l||(e.l={}),e.l[t+o]=n,n?r||e.addEventListener(t,o?de:me,o):e.removeEventListener(t,o?de:me,o);else if(t!=="dangerouslySetInnerHTML"){if(a)t=t.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(t!=="href"&&t!=="list"&&t!=="form"&&t!=="tabIndex"&&t!=="download"&&t in e)try{e[t]=n==null?"":n;break e}catch(c){}typeof n=="function"||(n!=null&&(n!==!1||t[0]==="a"&&t[1]==="r")?e.setAttribute(t,n):e.removeAttribute(t))}}function me(e){this.l[e.type+!1](f.event?f.event(e):e)}function de(e){this.l[e.type+!0](f.event?f.event(e):e)}function Q(e,t,n,r,a,o,c,u,m){var p,i,d,_,s,y,h,v,k,b,F,x=t.type;if(t.constructor!==void 0)return null;n.__h!=null&&(m=n.__h,u=t.__e=n.__e,t.__h=null,o=[u]),(p=f.__b)&&p(t);try{e:if(typeof x=="function"){if(v=t.props,k=(p=x.contextType)&&r[p.__c],b=p?k?k.props.value:p.__:r,n.__c?h=(i=t.__c=n.__c).__=i.__E:("prototype"in x&&x.prototype.render?t.__c=i=new x(v,b):(t.__c=i=new S(v,b),i.constructor=x,i.render=Te),k&&k.sub(i),i.props=v,i.state||(i.state={}),i.context=b,i.__n=r,d=i.__d=!0,i.__h=[]),i.__s==null&&(i.__s=i.state),x.getDerivedStateFromProps!=null&&(i.__s==i.state&&(i.__s=E({},i.__s)),E(i.__s,x.getDerivedStateFromProps(v,i.__s))),_=i.props,s=i.state,d)x.getDerivedStateFromProps==null&&i.componentWillMount!=null&&i.componentWillMount(),i.componentDidMount!=null&&i.__h.push(i.componentDidMount);else{if(x.getDerivedStateFromProps==null&&v!==_&&i.componentWillReceiveProps!=null&&i.componentWillReceiveProps(v,b),!i.__e&&i.shouldComponentUpdate!=null&&i.shouldComponentUpdate(v,i.__s,b)===!1||t.__v===n.__v){i.props=v,i.state=i.__s,t.__v!==n.__v&&(i.__d=!1),i.__v=t,t.__e=n.__e,t.__k=n.__k,t.__k.forEach(function(D){D&&(D.__=t)}),i.__h.length&&c.push(i);break e}i.componentWillUpdate!=null&&i.componentWillUpdate(v,i.__s,b),i.componentDidUpdate!=null&&i.__h.push(function(){i.componentDidUpdate(_,s,y)})}i.context=b,i.props=v,i.state=i.__s,(p=f.__r)&&p(t),i.__d=!1,i.__v=t,i.__P=e,p=i.render(i.props,i.state,i.context),i.state=i.__s,i.getChildContext!=null&&(r=E(E({},r),i.getChildContext())),d||i.getSnapshotBeforeUpdate==null||(y=i.getSnapshotBeforeUpdate(_,s)),F=p!=null&&p.type===I&&p.key==null?p.props.children:p,fe(e,Array.isArray(F)?F:[F],t,n,r,a,o,c,u,m),i.base=t.__e,t.__h=null,i.__h.length&&c.push(i),h&&(i.__E=i.__=null),i.__e=!1}else o==null&&t.__v===n.__v?(t.__k=n.__k,t.__e=n.__e):t.__e=Ae(n.__e,t,n,r,a,o,c,m);(p=f.diffed)&&p(t)}catch(D){t.__v=null,(m||o!=null)&&(t.__e=u,t.__h=!!m,o[o.indexOf(u)]=null),f.__e(D,t,n)}}function ae(e,t){f.__c&&f.__c(t,e),e.some(function(n){try{e=n.__h,n.__h=[],e.some(function(r){r.call(n)})}catch(r){f.__e(r,n.__v)}})}function Ae(e,t,n,r,a,o,c,u){var m,p,i,d,_=n.props,s=t.props,y=t.type,h=0;if(y==="svg"&&(a=!0),o!=null){for(;h<o.length;h++)if((m=o[h])&&(m===e||(y?m.localName==y:m.nodeType==3))){e=m,o[h]=null;break}}if(e==null){if(y===null)return document.createTextNode(s);e=a?document.createElementNS("http://www.w3.org/2000/svg",y):document.createElement(y,s.is&&s),o=null,u=!1}if(y===null)_===s||u&&e.data===s||(e.data=s);else{if(o=o&&z.slice.call(e.childNodes),p=(_=n.props||$).dangerouslySetInnerHTML,i=s.dangerouslySetInnerHTML,!u){if(o!=null)for(_={},d=0;d<e.attributes.length;d++)_[e.attributes[d].name]=e.attributes[d].value;(i||p)&&(i&&(p&&i.__html==p.__html||i.__html===e.innerHTML)||(e.innerHTML=i&&i.__html||""))}if(Me(e,s,_,a,u),i)t.__k=[];else if(h=t.props.children,fe(e,Array.isArray(h)?h:[h],t,n,r,a&&y!=="foreignObject",o,c,e.firstChild,u),o!=null)for(h=o.length;h--;)o[h]!=null&&re(o[h]);u||("value"in s&&(h=s.value)!==void 0&&(h!==e.value||y==="progress"&&!h)&&B(e,"value",h,_.value,!1),"checked"in s&&(h=s.checked)!==void 0&&h!==e.checked&&B(e,"checked",h,_.checked,!1))}return e}function ce(e,t,n){try{typeof e=="function"?e(t):e.current=t}catch(r){f.__e(r,n)}}function ue(e,t,n){var r,a,o;if(f.unmount&&f.unmount(e),(r=e.ref)&&(r.current&&r.current!==e.__e||ce(r,null,t)),n||typeof e.type=="function"||(n=(a=e.__e)!=null),e.__e=e.__d=void 0,(r=e.__c)!=null){if(r.componentWillUnmount)try{r.componentWillUnmount()}catch(c){f.__e(c,t)}r.base=r.__P=null}if(r=e.__k)for(o=0;o<r.length;o++)r[o]&&ue(r[o],t,n);a!=null&&re(a)}function Te(e,t,n){return this.constructor(e,n)}function V(e,t,n){var r,a,o;f.__&&f.__(e,t),a=(r=typeof n=="function")?null:n&&n.__k||t.__k,o=[],Q(t,e=(!r&&n||t).__k=l(I,null,[e]),a||$,$,t.ownerSVGElement!==void 0,!r&&n?[n]:a?null:t.firstChild?z.slice.call(t.childNodes):null,o,!r&&n?n:a?a.__e:t.firstChild,r),ae(o,e)}f={__e:function(e,t){for(var n,r,a;t=t.__;)if((n=t.__c)&&!n.__)try{if((r=n.constructor)&&r.getDerivedStateFromError!=null&&(n.setState(r.getDerivedStateFromError(e)),a=n.__d),n.componentDidCatch!=null&&(n.componentDidCatch(e),a=n.__d),a)return n.__E=n}catch(o){e=o}throw e},__v:0},S.prototype.setState=function(e,t){var n;n=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=E({},this.state),typeof e=="function"&&(e=e(E({},n),this.props)),e&&E(n,e),e!=null&&this.__v&&(t&&this.__h.push(t),le(this))},S.prototype.forceUpdate=function(e){this.__v&&(this.__e=!0,e&&this.__h.push(e),le(this))},S.prototype.render=I,P=[],ne=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,q.__r=0;var U,w,he,G=0,J=[],ge=f.__b,ve=f.__r,ye=f.diffed,ke=f.__c,we=f.unmount;function be(e,t){f.__h&&f.__h(w,e,G||t),G=0;var n=w.__H||(w.__H={__:[],__h:[]});return e>=n.__.length&&n.__.push({}),n.__[e]}function g(e){return G=1,Pe(xe,e)}function Pe(e,t,n){var r=be(U++,2);return r.t=e,r.__c||(r.__=[n?n(t):xe(void 0,t),function(a){var o=r.t(r.__[0],a);r.__[0]!==o&&(r.__=[o,r.__[1]],r.__c.setState({}))}],r.__c=w),r.__}function Ce(e,t){var n=be(U++,3);!f.__s&&He(n.__H,t)&&(n.__=e,n.__H=t,w.__H.__h.push(n))}function Fe(){J.forEach(function(e){if(e.__P)try{e.__H.__h.forEach(L),e.__H.__h.forEach(X),e.__H.__h=[]}catch(t){e.__H.__h=[],f.__e(t,e.__v)}}),J=[]}f.__b=function(e){w=null,ge&&ge(e)},f.__r=function(e){ve&&ve(e),U=0;var t=(w=e.__c).__H;t&&(t.__h.forEach(L),t.__h.forEach(X),t.__h=[])},f.diffed=function(e){ye&&ye(e);var t=e.__c;t&&t.__H&&t.__H.__h.length&&(J.push(t)!==1&&he===f.requestAnimationFrame||((he=f.requestAnimationFrame)||function(n){var r,a=function(){clearTimeout(o),je&&cancelAnimationFrame(r),setTimeout(n)},o=setTimeout(a,100);je&&(r=requestAnimationFrame(a))})(Fe)),w=void 0},f.__c=function(e,t){t.some(function(n){try{n.__h.forEach(L),n.__h=n.__h.filter(function(r){return!r.__||X(r)})}catch(r){t.some(function(a){a.__h&&(a.__h=[])}),t=[],f.__e(r,n.__v)}}),ke&&ke(e,t)},f.unmount=function(e){we&&we(e);var t=e.__c;if(t&&t.__H)try{t.__H.__.forEach(L)}catch(n){f.__e(n,t.__v)}};var je=typeof requestAnimationFrame=="function";function L(e){var t=w;typeof e.__c=="function"&&e.__c(),w=t}function X(e){var t=w;e.__c=e.__(),w=t}function He(e,t){return!e||e.length!==t.length||t.some(function(n,r){return n!==e[r]})}function xe(e,t){return typeof t=="function"?t(e):t}async function Ne(e,t){let n=window.__xsrf_token;return await(await fetch("/graphql",{method:"POST",credentials:"include",headers:{"Content-Type":"application/json","X-XSRF-Token":n},body:JSON.stringify({query:e,variables:t})})).json()}function C([e]){return e}function R(e,t){let[n,r]=g(0),[a,o]=g(null),[c,u]=g(!0),[m,p]=g(null);if(m!=null)throw m;let i=()=>{u(!0),r(n+1)};return Ce(()=>{Ne(e,t).then(({data:d,errors:_})=>{u(!1),_!=null?p(_):o(d)}).catch(d=>p(d))},[e,t,n]),{data:a,loading:c,refetch:i}}function A(e,{onCommit:t,onError:n}){let[r,a]=g(!1),[o,c]=g(void 0);if(o!==void 0)throw o;return{commit:m=>{a(!0),Ne(e,m).then(({data:p,errors:i})=>{a(!1),i!=null?typeof n=="function"?n(i):c(i):t(p)}).catch(p=>c(p))},inFlight:r}}function M({size:e="small",style:t=""}){return l("svg",{xmlns:"http://www.w3.org/2000/svg",class:`${e==="large"?"h-8 w-8":"h-4 w-4"} ${t!=null?t:""}`,viewBox:"0 0 24 24",stroke:"none",fill:"currentColor"},l("rect",{x:"10",y:"0",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 6.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.125s",repeatCount:"indefinite"})),l("rect",{x:"16",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.25s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 17.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.375s",repeatCount:"indefinite"})),l("rect",{x:"10",y:"16",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.5s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 17.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.625s",repeatCount:"indefinite"})),l("rect",{x:"0",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.75s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 6.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.875s",repeatCount:"indefinite"})))}function j({title:e,onClick:t,disabled:n=!1,loading:r=!1,type:a="default",style:o}){let c=`h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${o!=null?o:""}`;switch(a){case"flat":c+=" bg-white text-black",n||(c+=" hover:bg-gray-200");break;case"default":default:c+=" bg-blue-500 text-white",n||(c+=" hover:bg-blue-400")}return n===!0&&(c+=" opacity-80 cursor-not-allowed"),l("button",{class:c,onClick:n===!0?void 0:t},l("div",null,e),r===!0&&l(M,null))}function De(){window.location.href=window.location.href}function $e(e){let t={};for(let n of Object.getOwnPropertyNames(e))t[n]=e[n];return JSON.stringify(t)}function Oe({error:e}){return l("div",{class:"w-full flex justify-center p-8"},l("div",{class:"w-full max-w-3xl bg-red-500 rounded-lg p-4"},l("h1",{class:"text-white text-xl font-semibold"},"Error"),l("p",{class:"text-red-100"},"Sorry, something went wrong."),l("pre",{class:"text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap break-all"},$e(e)),l(j,{title:"Reload Page",onClick:De,type:"flat",style:"w-full mt-2"})))}var K=class extends S{constructor(){super();this.state={error:!1}}componentDidCatch(t){console.error(t),this.setState({error:t})}render({children:t},{error:n}){return n!==!1?l(Oe,{error:n}):t}},Ee=K;var qe=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M5 15l7-7 7 7"})),Be=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M19 9l-7 7-7-7"}));function W({title:e,children:t,initiallyExpanded:n=!0}){let[r,a]=g(n);return l("div",{class:"w-full mt-2 rounded-md bg-gray-100 dark:bg-gray-700"},l("button",{class:"w-full p-2 flex justify-between items-center rounded-md hover:bg-gray-200 dark:hover:bg-gray-600",onClick:()=>a(!r)},l("h1",{class:"text-lg font-semibold"},e),r?qe:Be),l("div",{class:"w-full px-2 pb-2",style:{display:r?void 0:"none"}},t))}var Le=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"})),Ie=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}));function N({type:e="info",message:t,style:n}){let r=`flex align-start p-3 rounded-md block font-semibold ${n!=null?n:""}`,a;switch(e){case"error":r+=" bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",a=Ie;break;case"warning":r+=" bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",a=Ie;break;case"info":default:r+=" bg-blue-200 text-blue-800 dark:bg-blue-800 dark:text-blue-200",a=Le}return l("div",{class:r},l("div",{class:"mr-2"},a),l("p",{class:"leading-snug"},t))}var Re=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-10 w-10",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z"}));function Y({id:e,token:t,claimedBy:n}){return l("div",{class:"w-full mb-2 rounded-xl overflow-hidden"},l("div",{class:`
          bg-yellow-400 text-yellow-900
          divide-dashed divide-y divide-yellow-600 divide-y-2
        `},l("div",{class:"flex p-4 items-center"},Re,l("h1",{class:"ml-4 text-xl font-semibold"},"Account Registration Code ",l("span",{class:"text-yellow-600"},"admits one"))),n==null?l("div",{class:"p-4"},l("div",{class:"w-full font-mono text-center select-all break-all p-2 bg-yellow-500 rounded-md"},t)):l("div",null)),n!=null&&l("p",{class:"p-4 font-semibold text-center text-gray-500"},"Claimed by ",n==null?void 0:n.name))}function Z({}){var m,p,i,d;let{data:e,loading:t,refetch:n}=R(C`
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
  `),r=(d=(i=(p=(m=e==null?void 0:e.viewer)==null?void 0:m.invites)==null?void 0:p.edges)==null?void 0:i.map(({node:_})=>_))!=null?d:[],[a,o]=g(null),{commit:c,inFlight:u}=A(C`
    mutation IssueInvite {
      issueInvite {
        id
      }
    }
  `,{onCommit:n,onError:([{message:_}])=>o(_)});return l(I,null,t?l(M,{size:"large",style:"my-4 mx-auto"}):r.map(_=>l(Y,{..._})),a&&l(N,{message:a,type:"error",style:"my-2"}),l(j,{title:"Issue new Invitation",type:"flat",onClick:c,loading:u,disabled:u,style:"w-full"}))}function T({value:e,onChange:t,type:n,label:r,placeholder:a,style:o}){let[c]=g(`input-id-${Math.floor(Math.random()*1e4)}`);return l("div",{class:`w-full ${o!=null?o:""}`},l("label",{class:"text-gray-500 italic",for:c},r),l("input",{class:"w-full p-2 text-md rounded-md bg-gray-200 dark:bg-gray-600 text-black dark:text-white",id:c,type:n!=null?n:"text",value:e,onInput:u=>typeof t=="function"&&t(u.target.value),placeholder:a}))}function ee({currentName:e}){let[t,n]=g(e),[r,a]=g(null),[o,c]=g(null),{commit:u,inFlight:m}=A(C`
    mutation ChangeName($name: String!) {
      updateUser(input: {name: $name}) {
        id
        user {
          id
          name
        }
      }
    }
  `,{onCommit:({updateUser:d})=>{var _,s;c(`Name changed to ${(_=d==null?void 0:d.user)==null?void 0:_.name}`),n((s=d==null?void 0:d.user)==null?void 0:s.name),a(null)},onError:([{message:d}])=>{a(`Failed to change email: ${d}`),c(null)}}),p=!m&&t.trim().length>0,i=d=>{d.preventDefault(),p&&u({name:t.trim()})};return l("form",{onSubmit:i},r&&l(N,{message:r,type:"error",style:"mb-2"}),o&&l(N,{message:o,style:"mb-2"}),l(T,{label:"Public profile name",placeholder:"Ada Lovelace",value:t,onChange:n,style:"mb-2"}),l(j,{title:"Update",onClick:i,disabled:!p,loading:m,style:"w-full"}))}function te({}){let[e,t]=g(""),[n,r]=g(""),[a,o]=g(null),[c,u]=g(null),{commit:m,inFlight:p}=A(C`
    mutation ChangeEmailMutation($email: String!) {
      updateUser(input: {email: $email}) {
        id
        email
      }
    }
  `,{onCommit:({updateUser:_})=>{t(""),r(""),o(null),u(`Email changed to ${_==null?void 0:_.email}`)},onError:([{message:_}])=>{o(`Failed to change email: ${_}`),u(null)}}),i=!p&&e.trim().length>0&&e===n,d=_=>{_.preventDefault(),i&&m({email:e.trim()})};return l("form",{onSubmit:d},a&&l(N,{message:a,type:"error",style:"mb-2"}),c&&l(N,{message:c,style:"mb-2"}),l(T,{label:"New email address",placeholder:"ada.lovelace@urls.fyi",type:"email",value:e,onChange:t,style:"mb-2"}),l(T,{label:"Confirm email address",placeholder:"ada.lovelace@urls.fyi",type:"email",value:n,onChange:r,style:"mb-2"}),l(j,{title:"Update",onClick:d,disabled:!i,loading:p,style:"w-full"}))}function We(){var n,r,a,o;let{data:e,loading:t}=R(C`
    query AccountQuery {
      viewer {
        id
        user {
          id
          name
        }
      }
    }
  `);return l("div",{class:"w-full flex justify-center p-8"},t?l(M,{size:"large"}):l("div",{class:"w-full max-w-screen-md bg-white dark:bg-gray-800 shadow rounded-lg p-4"},l("h1",{class:"text-2xl font-semibold"},"Account Settings"),l("h2",{class:"text-xl text-gray-500 mb-4"},"Welcome back ",(r=(n=e==null?void 0:e.viewer)==null?void 0:n.user)==null?void 0:r.name),l(W,{title:"Change name",initiallyExpanded:!1},l(ee,{currentName:(o=(a=e==null?void 0:e.viewer)==null?void 0:a.user)==null?void 0:o.name})),l(W,{title:"Change email",initiallyExpanded:!1},l(te,null)),l(W,{title:"Invite a friend"},l(Z,null))))}V(l(Ee,null,l(We,null)),document.getElementById("account"));
//# sourceMappingURL=account.js.map
