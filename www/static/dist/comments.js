var u,N,Y,K,F={},U=[],Ne=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function C(t,e){for(var n in e)t[n]=e[n];return t}function Z(t){var e=t.parentNode;e&&e.removeChild(t)}function l(t,e,n){var r,i,o,c=arguments,f={};for(o in e)o=="key"?r=e[o]:o=="ref"?i=e[o]:f[o]=e[o];if(arguments.length>3)for(n=[n],o=3;o<arguments.length;o++)n.push(c[o]);if(n!=null&&(f.children=n),typeof t=="function"&&t.defaultProps!=null)for(o in t.defaultProps)f[o]===void 0&&(f[o]=t.defaultProps[o]);return M(t,f,r,i,null)}function M(t,e,n,r,i){var o={type:t,props:e,key:n,ref:r,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:i==null?++u.__v:i};return u.vnode!=null&&u.vnode(o),o}function j(t){return t.children}function S(t,e){this.props=t,this.context=e}function E(t,e){if(e==null)return t.__?E(t.__,t.__.__k.indexOf(t)+1):null;for(var n;e<t.__k.length;e++)if((n=t.__k[e])!=null&&n.__e!=null)return n.__e;return typeof t.type=="function"?E(t):null}function ee(t){var e,n;if((t=t.__)!=null&&t.__c!=null){for(t.__e=t.__c.base=null,e=0;e<t.__k.length;e++)if((n=t.__k[e])!=null&&n.__e!=null){t.__e=t.__c.base=n.__e;break}return ee(t)}}function te(t){(!t.__d&&(t.__d=!0)&&N.push(t)&&!P.__r++||K!==u.debounceRendering)&&((K=u.debounceRendering)||Y)(P)}function P(){for(var t;P.__r=N.length;)t=N.sort(function(e,n){return e.__v.__b-n.__v.__b}),N=[],t.some(function(e){var n,r,i,o,c,f;e.__d&&(c=(o=(n=e).__v).__e,(f=n.__P)&&(r=[],(i=C({},o)).__v=o.__v+1,z(f,o,i,n.__n,f.ownerSVGElement!==void 0,o.__h!=null?[c]:null,r,c==null?E(o):c,o.__h),ne(r,o),o.__e!=c&&ee(o)))})}function le(t,e,n,r,i,o,c,f,m,p){var _,d,s,a,v,h,g,y=r&&r.__k||U,k=y.length;for(n.__k=[],_=0;_<e.length;_++)if((a=n.__k[_]=(a=e[_])==null||typeof a=="boolean"?null:typeof a=="string"||typeof a=="number"||typeof a=="bigint"?M(null,a,null,null,a):Array.isArray(a)?M(j,{children:a},null,null,null):a.__b>0?M(a.type,a.props,a.key,null,a.__v):a)!=null){if(a.__=n,a.__b=n.__b+1,(s=y[_])===null||s&&a.key==s.key&&a.type===s.type)y[_]=void 0;else for(d=0;d<k;d++){if((s=y[d])&&a.key==s.key&&a.type===s.type){y[d]=void 0;break}s=null}z(t,a,s=s||F,i,o,c,f,m,p),v=a.__e,(d=a.ref)&&s.ref!=d&&(g||(g=[]),s.ref&&g.push(s.ref,null,a),g.push(d,a.__c||v,a)),v!=null?(h==null&&(h=v),typeof a.type=="function"&&a.__k!=null&&a.__k===s.__k?a.__d=m=re(a,m,t):m=oe(t,a,s,y,v,m),p||n.type!=="option"?typeof n.type=="function"&&(n.__d=m):t.value=""):m&&s.__e==m&&m.parentNode!=t&&(m=E(s))}for(n.__e=h,_=k;_--;)y[_]!=null&&(typeof n.type=="function"&&y[_].__e!=null&&y[_].__e==n.__d&&(n.__d=E(r,_+1)),ie(y[_],y[_]));if(g)for(_=0;_<g.length;_++)_e(g[_],g[++_],g[++_])}function re(t,e,n){var r,i;for(r=0;r<t.__k.length;r++)(i=t.__k[r])&&(i.__=t,e=typeof i.type=="function"?re(i,e,n):oe(n,i,i,t.__k,i.__e,e));return e}function oe(t,e,n,r,i,o){var c,f,m;if(e.__d!==void 0)c=e.__d,e.__d=void 0;else if(n==null||i!=o||i.parentNode==null)e:if(o==null||o.parentNode!==t)t.appendChild(i),c=null;else{for(f=o,m=0;(f=f.nextSibling)&&m<r.length;m+=2)if(f==i)break e;t.insertBefore(i,o),c=o}return c!==void 0?c:i.nextSibling}function Ee(t,e,n,r,i){var o;for(o in n)o==="children"||o==="key"||o in e||B(t,o,null,n[o],r);for(o in e)i&&typeof e[o]!="function"||o==="children"||o==="key"||o==="value"||o==="checked"||n[o]===e[o]||B(t,o,e[o],n[o],r)}function ae(t,e,n){e[0]==="-"?t.setProperty(e,n):t[e]=n==null?"":typeof n!="number"||Ne.test(e)?n:n+"px"}function B(t,e,n,r,i){var o;e:if(e==="style")if(typeof n=="string")t.style.cssText=n;else{if(typeof r=="string"&&(t.style.cssText=r=""),r)for(e in r)n&&e in n||ae(t.style,e,"");if(n)for(e in n)r&&n[e]===r[e]||ae(t.style,e,n[e])}else if(e[0]==="o"&&e[1]==="n")o=e!==(e=e.replace(/Capture$/,"")),e=e.toLowerCase()in t?e.toLowerCase().slice(2):e.slice(2),t.l||(t.l={}),t.l[e+o]=n,n?r||t.addEventListener(e,o?ce:se,o):t.removeEventListener(e,o?ce:se,o);else if(e!=="dangerouslySetInnerHTML"){if(i)e=e.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(e!=="href"&&e!=="list"&&e!=="form"&&e!=="tabIndex"&&e!=="download"&&e in t)try{t[e]=n==null?"":n;break e}catch(c){}typeof n=="function"||(n!=null&&(n!==!1||e[0]==="a"&&e[1]==="r")?t.setAttribute(e,n):t.removeAttribute(e))}}function se(t){this.l[t.type+!1](u.event?u.event(t):t)}function ce(t){this.l[t.type+!0](u.event?u.event(t):t)}function z(t,e,n,r,i,o,c,f,m){var p,_,d,s,a,v,h,g,y,k,T,b=e.type;if(e.constructor!==void 0)return null;n.__h!=null&&(m=n.__h,f=e.__e=n.__e,e.__h=null,o=[f]),(p=u.__b)&&p(e);try{e:if(typeof b=="function"){if(g=e.props,y=(p=b.contextType)&&r[p.__c],k=p?y?y.props.value:p.__:r,n.__c?h=(_=e.__c=n.__c).__=_.__E:("prototype"in b&&b.prototype.render?e.__c=_=new b(g,k):(e.__c=_=new S(g,k),_.constructor=b,_.render=Ae),y&&y.sub(_),_.props=g,_.state||(_.state={}),_.context=k,_.__n=r,d=_.__d=!0,_.__h=[]),_.__s==null&&(_.__s=_.state),b.getDerivedStateFromProps!=null&&(_.__s==_.state&&(_.__s=C({},_.__s)),C(_.__s,b.getDerivedStateFromProps(g,_.__s))),s=_.props,a=_.state,d)b.getDerivedStateFromProps==null&&_.componentWillMount!=null&&_.componentWillMount(),_.componentDidMount!=null&&_.__h.push(_.componentDidMount);else{if(b.getDerivedStateFromProps==null&&g!==s&&_.componentWillReceiveProps!=null&&_.componentWillReceiveProps(g,k),!_.__e&&_.shouldComponentUpdate!=null&&_.shouldComponentUpdate(g,_.__s,k)===!1||e.__v===n.__v){_.props=g,_.state=_.__s,e.__v!==n.__v&&(_.__d=!1),_.__v=e,e.__e=n.__e,e.__k=n.__k,e.__k.forEach(function(H){H&&(H.__=e)}),_.__h.length&&c.push(_);break e}_.componentWillUpdate!=null&&_.componentWillUpdate(g,_.__s,k),_.componentDidUpdate!=null&&_.__h.push(function(){_.componentDidUpdate(s,a,v)})}_.context=k,_.props=g,_.state=_.__s,(p=u.__r)&&p(e),_.__d=!1,_.__v=e,_.__P=t,p=_.render(_.props,_.state,_.context),_.state=_.__s,_.getChildContext!=null&&(r=C(C({},r),_.getChildContext())),d||_.getSnapshotBeforeUpdate==null||(v=_.getSnapshotBeforeUpdate(s,a)),T=p!=null&&p.type===j&&p.key==null?p.props.children:p,le(t,Array.isArray(T)?T:[T],e,n,r,i,o,c,f,m),_.base=e.__e,e.__h=null,_.__h.length&&c.push(_),h&&(_.__E=_.__=null),_.__e=!1}else o==null&&e.__v===n.__v?(e.__k=n.__k,e.__e=n.__e):e.__e=Ie(n.__e,e,n,r,i,o,c,m);(p=u.diffed)&&p(e)}catch(H){e.__v=null,(m||o!=null)&&(e.__e=f,e.__h=!!m,o[o.indexOf(f)]=null),u.__e(H,e,n)}}function ne(t,e){u.__c&&u.__c(e,t),t.some(function(n){try{t=n.__h,n.__h=[],t.some(function(r){r.call(n)})}catch(r){u.__e(r,n.__v)}})}function Ie(t,e,n,r,i,o,c,f){var m,p,_,d,s=n.props,a=e.props,v=e.type,h=0;if(v==="svg"&&(i=!0),o!=null){for(;h<o.length;h++)if((m=o[h])&&(m===t||(v?m.localName==v:m.nodeType==3))){t=m,o[h]=null;break}}if(t==null){if(v===null)return document.createTextNode(a);t=i?document.createElementNS("http://www.w3.org/2000/svg",v):document.createElement(v,a.is&&a),o=null,f=!1}if(v===null)s===a||f&&t.data===a||(t.data=a);else{if(o=o&&U.slice.call(t.childNodes),p=(s=n.props||F).dangerouslySetInnerHTML,_=a.dangerouslySetInnerHTML,!f){if(o!=null)for(s={},d=0;d<t.attributes.length;d++)s[t.attributes[d].name]=t.attributes[d].value;(_||p)&&(_&&(p&&_.__html==p.__html||_.__html===t.innerHTML)||(t.innerHTML=_&&_.__html||""))}if(Ee(t,a,s,i,f),_)e.__k=[];else if(h=e.props.children,le(t,Array.isArray(h)?h:[h],e,n,r,i&&v!=="foreignObject",o,c,t.firstChild,f),o!=null)for(h=o.length;h--;)o[h]!=null&&Z(o[h]);f||("value"in a&&(h=a.value)!==void 0&&(h!==t.value||v==="progress"&&!h)&&B(t,"value",h,s.value,!1),"checked"in a&&(h=a.checked)!==void 0&&h!==t.checked&&B(t,"checked",h,s.checked,!1))}return t}function _e(t,e,n){try{typeof t=="function"?t(e):t.current=e}catch(r){u.__e(r,n)}}function ie(t,e,n){var r,i,o;if(u.unmount&&u.unmount(t),(r=t.ref)&&(r.current&&r.current!==t.__e||_e(r,null,e)),n||typeof t.type=="function"||(n=(i=t.__e)!=null),t.__e=t.__d=void 0,(r=t.__c)!=null){if(r.componentWillUnmount)try{r.componentWillUnmount()}catch(c){u.__e(c,e)}r.base=r.__P=null}if(r=t.__k)for(o=0;o<r.length;o++)r[o]&&ie(r[o],e,n);i!=null&&Z(i)}function Ae(t,e,n){return this.constructor(t,n)}function R(t,e,n){var r,i,o;u.__&&u.__(t,e),i=(r=typeof n=="function")?null:n&&n.__k||e.__k,o=[],z(e,t=(!r&&n||e).__k=l(j,null,[t]),i||F,F,e.ownerSVGElement!==void 0,!r&&n?[n]:i?null:e.firstChild?U.slice.call(e.childNodes):null,o,!r&&n?n:i?i.__e:e.firstChild,r),ne(o,t)}u={__e:function(t,e){for(var n,r,i;e=e.__;)if((n=e.__c)&&!n.__)try{if((r=n.constructor)&&r.getDerivedStateFromError!=null&&(n.setState(r.getDerivedStateFromError(t)),i=n.__d),n.componentDidCatch!=null&&(n.componentDidCatch(t),i=n.__d),i)return n.__E=n}catch(o){t=o}throw t},__v:0},S.prototype.setState=function(t,e){var n;n=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=C({},this.state),typeof t=="function"&&(t=t(C({},n),this.props)),t&&C(n,t),t!=null&&this.__v&&(e&&this.__h.push(e),te(this))},S.prototype.forceUpdate=function(t){this.__v&&(this.__e=!0,t&&this.__h.push(t),te(this))},S.prototype.render=j,N=[],Y=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,P.__r=0;var q,w,ue,W=0,V=[],fe=u.__b,pe=u.__r,me=u.diffed,de=u.__c,he=u.unmount;function ge(t,e){u.__h&&u.__h(w,t,W||e),W=0;var n=w.__H||(w.__H={__:[],__h:[]});return t>=n.__.length&&n.__.push({}),n.__[t]}function x(t){return W=1,De(ve,t)}function De(t,e,n){var r=ge(q++,2);return r.t=t,r.__c||(r.__=[n?n(e):ve(void 0,e),function(i){var o=r.t(r.__[0],i);r.__[0]!==o&&(r.__=[o,r.__[1]],r.__c.setState({}))}],r.__c=w),r.__}function ye(t,e){var n=ge(q++,3);!u.__s&&Te(n.__H,e)&&(n.__=t,n.__H=e,w.__H.__h.push(n))}function He(){V.forEach(function(t){if(t.__P)try{t.__H.__h.forEach($),t.__H.__h.forEach(J),t.__H.__h=[]}catch(e){t.__H.__h=[],u.__e(e,t.__v)}}),V=[]}u.__b=function(t){w=null,fe&&fe(t)},u.__r=function(t){pe&&pe(t),q=0;var e=(w=t.__c).__H;e&&(e.__h.forEach($),e.__h.forEach(J),e.__h=[])},u.diffed=function(t){me&&me(t);var e=t.__c;e&&e.__H&&e.__H.__h.length&&(V.push(e)!==1&&ue===u.requestAnimationFrame||((ue=u.requestAnimationFrame)||function(n){var r,i=function(){clearTimeout(o),we&&cancelAnimationFrame(r),setTimeout(n)},o=setTimeout(i,100);we&&(r=requestAnimationFrame(i))})(He)),w=void 0},u.__c=function(t,e){e.some(function(n){try{n.__h.forEach($),n.__h=n.__h.filter(function(r){return!r.__||J(r)})}catch(r){e.some(function(i){i.__h&&(i.__h=[])}),e=[],u.__e(r,n.__v)}}),de&&de(t,e)},u.unmount=function(t){he&&he(t);var e=t.__c;if(e&&e.__H)try{e.__H.__.forEach($)}catch(n){u.__e(n,e.__v)}};var we=typeof requestAnimationFrame=="function";function $(t){var e=w;typeof t.__c=="function"&&t.__c(),w=e}function J(t){var e=w;t.__c=t.__(),w=e}function Te(t,e){return!t||t.length!==e.length||e.some(function(n,r){return n!==t[r]})}function ve(t,e){return typeof e=="function"?e(t):e}async function ke(t,e){let n=window.__xsrf_token;return await(await fetch("/graphql",{method:"POST",credentials:"include",headers:{"Content-Type":"application/json","X-XSRF-Token":n},body:JSON.stringify({query:t,variables:e})})).json()}function O([t]){return t}function xe(t,e){let[n,r]=x(0),[i,o]=x(null),[c,f]=x(!0),[m,p]=x(null);if(m!=null)throw m;let _=()=>{f(!0),r(n+1)};return ye(()=>{ke(t,e).then(({data:d,errors:s})=>{f(!1),s!=null?p(s):o(d)}).catch(d=>p(d))},[t,n]),{data:i,loading:c,refetch:_}}function be(t,{onCommit:e,onError:n}){let[r,i]=x(!1),[o,c]=x(void 0);if(o!==void 0)throw o;return{commit:m=>{i(!0),ke(t,m).then(({data:p,errors:_})=>{i(!1),_!=null?typeof n=="function"?n(_):c(_):e(p)}).catch(p=>c(p))},inFlight:r}}function I({size:t="small",style:e=""}){return l("svg",{xmlns:"http://www.w3.org/2000/svg",class:`${t==="large"?"h-8 w-8":"h-4 w-4"} ${e!=null?e:""}`,viewBox:"0 0 24 24",stroke:"none",fill:"currentColor"},l("rect",{x:"10",y:"0",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 6.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.125s",repeatCount:"indefinite"})),l("rect",{x:"16",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.25s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 17.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.375s",repeatCount:"indefinite"})),l("rect",{x:"10",y:"16",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.5s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 17.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.625s",repeatCount:"indefinite"})),l("rect",{x:"0",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.75s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 6.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.875s",repeatCount:"indefinite"})))}function A({title:t,onClick:e,disabled:n=!1,loading:r=!1,type:i="default",style:o}){let c=`h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${o!=null?o:""}`;switch(i){case"flat":c+=" bg-white text-black",n||(c+=" hover:bg-gray-200");break;case"default":default:c+=" bg-blue-500 text-white",n||(c+=" hover:bg-blue-400")}return n===!0&&(c+=" opacity-80 cursor-not-allowed"),l("button",{class:c,onClick:n===!0?void 0:e},l("div",null,t),r===!0&&l(I,null))}function Fe(){window.location.href=window.location.href}function Me(t){let e={};for(let n of Object.getOwnPropertyNames(t))e[n]=t[n];return JSON.stringify(e)}function Pe({error:t}){return l("div",{class:"w-full flex justify-center p-8"},l("div",{class:"w-full max-w-3xl bg-red-500 rounded-lg p-4"},l("h1",{class:"text-white text-xl font-semibold"},"Error"),l("p",{class:"text-red-100"},"Sorry, something went wrong."),l("pre",{class:"text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap break-all"},Me(t)),l(A,{title:"Reload Page",onClick:Fe,type:"flat",style:"w-full mt-2"})))}var Q=class extends S{constructor(){super();this.state={error:!1}}componentDidCatch(e){console.error(e),this.setState({error:e})}render({children:e},{error:n}){return n!==!1?l(Pe,{error:n}):e}},Ce=Q;function L({icon:t,text:e}){return l("div",{class:"flex items-center"},t," ",e)}var Be=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-4 w-4 mr-1",viewBox:"0 0 20 20",fill:"currentColor"},l("path",{"fill-rule":"evenodd",d:"M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zm-2 4a5 5 0 00-4.546 2.916A5.986 5.986 0 0010 16a5.986 5.986 0 004.546-2.084A5 5 0 0010 11z","clip-rule":"evenodd"})),$e=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-4 w-4 mr-1",viewBox:"0 0 20 20",fill:"currentColor"},l("path",{"fill-rule":"evenodd",d:"M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z","clip-rule":"evenodd"}));function Oe(t){let e=["Sunday","Monday","Tuesday","Wednesday","Thursday","Friday","Saturday"],n=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],r=new Date(t);return`${e[r.getDay()]} ${r.getDate()}. ${n[r.getMonth()]} ${r.getFullYear()}`}function D({id:t,html:e,createdAt:n,createdBy:r}){return l("div",{class:"w-full"},l("div",{dangerouslySetInnerHTML:{__html:e}}),l("div",{class:`
          flex justify-start items-center
          text-sm italic text-gray-400 dark:text-gray-500
          space-x-1
        `},l(L,{icon:Be,text:r.name}),l("span",{class:"sm:block hidden"},"·"),l(L,{icon:$e,text:Oe(n)})))}var Le=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"})),je=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}));function G({type:t="info",message:e,style:n}){let r=`flex align-start p-3 rounded-md block font-semibold ${n!=null?n:""}`,i;switch(t){case"error":r+=" bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",i=je;break;case"warning":r+=" bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",i=je;break;case"info":default:r+=" bg-blue-200 text-blue-800 dark:bg-blue-800 dark:text-blue-200",i=Le}return l("div",{class:r},l("div",{class:"mr-2"},i),l("p",{class:"leading-snug"},e))}var Ue=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-5 w-5 mr-2",viewBox:"0 0 20 20",fill:"currentColor"},l("path",{d:"M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z"}));function X({urlID:t,repliesToID:e}){let[n,r]=x(""),[i,o]=x(null),[c,f]=x([]),{commit:m,inFlight:p}=be(O`
    mutation CommentInputMutation($text: String!, $url: ID!) {
      comment(input: { comment: $text, url: $url }) {
        ...CommentFragment
      }
    }

    fragment CommentFragment on Comment {
      id
      html
      createdAt
      createdBy {
        id
        name
      }
    }
  `,{onCommit:({comment:s})=>{r(""),o(null),f([...c,s])},onError:([{message:s}])=>{o(s)}}),_=n.trim().length==0||p,d=s=>{s.preventDefault(),!_&&m({text:n.trim(),url:t})};return l(j,null,c.map(s=>l(D,{...s})),l("form",{class:"w-full",onSubmit:d},i&&l(G,{message:i,type:"error",style:"mb-2"}),l("div",{class:"w-full p-2 rounded bg-gray-200 dark:bg-gray-600"},l("textarea",{class:"w-full h-14 resize-none bg-transparent leading-none",placeholder:"Your thoughts, formatted with *markdown* ...",onInput:s=>r(s.target.value),value:n},n),l(A,{title:l("div",{class:"flex items-center"},Ue," Comment"),onClick:d,disabled:_,loading:p}))))}function ze({urlID:t}){var r,i,o,c,f,m,p,_,d;let{data:e,loading:n}=xe(O`
    query CommentsQuery($url: ID!) {
      viewer {
        user {
          id
        }
      }
      url: fetch__Url(id: $url) {
        id
        comments {
          edges {
            node {
              ...CommentFragment
            }
          }
        }
      }
    }

    fragment CommentFragment on Comment {
      id
      html
      createdAt
      createdBy {
        id
        name
      }
    }
  `,{url:t});return l("div",{class:"w-full flex flex-col items-center justify-center space-y-1 sm:pl-14"},((c=(o=(i=(r=e==null?void 0:e.url)==null?void 0:r.comments)==null?void 0:i.edges)==null?void 0:o.length)!=null?c:0)>0&&l("h2",{class:"w-full text-xl font-semibold"},"Comments"),n?l(I,{size:"large"}):(p=(m=(f=e==null?void 0:e.url)==null?void 0:f.comments)==null?void 0:m.edges)==null?void 0:p.map(({node:s})=>l(D,{...s})),((d=(_=e==null?void 0:e.viewer)==null?void 0:_.user)==null?void 0:d.id)&&l(X,{urlID:t}))}var Se=document.getElementById("comments");R(l(Ce,null,l(ze,{urlID:Se.dataset.urlId})),Se);
//# sourceMappingURL=comments.js.map
