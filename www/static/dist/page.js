var c,N,Y,K,A={},R=[],wt=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function C(t,e){for(var n in e)t[n]=e[n];return t}function Z(t){var e=t.parentNode;e&&e.removeChild(t)}function l(t,e,n){var r,_,o,a=arguments,f={};for(o in e)o=="key"?r=e[o]:o=="ref"?_=e[o]:f[o]=e[o];if(arguments.length>3)for(n=[n],o=3;o<arguments.length;o++)n.push(a[o]);if(n!=null&&(f.children=n),typeof t=="function"&&t.defaultProps!=null)for(o in t.defaultProps)f[o]===void 0&&(f[o]=t.defaultProps[o]);return B(t,f,r,_,null)}function B(t,e,n,r,_){var o={type:t,props:e,key:n,ref:r,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:_==null?++c.__v:_};return c.vnode!=null&&c.vnode(o),o}function j(t){return t.children}function E(t,e){this.props=t,this.context=e}function M(t,e){if(e==null)return t.__?M(t.__,t.__.__k.indexOf(t)+1):null;for(var n;e<t.__k.length;e++)if((n=t.__k[e])!=null&&n.__e!=null)return n.__e;return typeof t.type=="function"?M(t):null}function tt(t){var e,n;if((t=t.__)!=null&&t.__c!=null){for(t.__e=t.__c.base=null,e=0;e<t.__k.length;e++)if((n=t.__k[e])!=null&&n.__e!=null){t.__e=t.__c.base=n.__e;break}return tt(t)}}function et(t){(!t.__d&&(t.__d=!0)&&N.push(t)&&!$.__r++||K!==c.debounceRendering)&&((K=c.debounceRendering)||Y)($)}function $(){for(var t;$.__r=N.length;)t=N.sort(function(e,n){return e.__v.__b-n.__v.__b}),N=[],t.some(function(e){var n,r,_,o,a,f;e.__d&&(a=(o=(n=e).__v).__e,(f=n.__P)&&(r=[],(_=C({},o)).__v=o.__v+1,q(f,o,_,n.__n,f.ownerSVGElement!==void 0,o.__h!=null?[a]:null,r,a==null?M(o):a,o.__h),nt(r,o),o.__e!=a&&tt(o)))})}function lt(t,e,n,r,_,o,a,f,d,p){var i,g,u,s,m,h,v,y=r&&r.__k||R,k=y.length;for(n.__k=[],i=0;i<e.length;i++)if((s=n.__k[i]=(s=e[i])==null||typeof s=="boolean"?null:typeof s=="string"||typeof s=="number"||typeof s=="bigint"?B(null,s,null,null,s):Array.isArray(s)?B(j,{children:s},null,null,null):s.__b>0?B(s.type,s.props,s.key,null,s.__v):s)!=null){if(s.__=n,s.__b=n.__b+1,(u=y[i])===null||u&&s.key==u.key&&s.type===u.type)y[i]=void 0;else for(g=0;g<k;g++){if((u=y[g])&&s.key==u.key&&s.type===u.type){y[g]=void 0;break}u=null}q(t,s,u=u||A,_,o,a,f,d,p),m=s.__e,(g=s.ref)&&u.ref!=g&&(v||(v=[]),u.ref&&v.push(u.ref,null,s),v.push(g,s.__c||m,s)),m!=null?(h==null&&(h=m),typeof s.type=="function"&&s.__k!=null&&s.__k===u.__k?s.__d=d=ot(s,d,t):d=rt(t,s,u,y,m,d),p||n.type!=="option"?typeof n.type=="function"&&(n.__d=d):t.value=""):d&&u.__e==d&&d.parentNode!=t&&(d=M(u))}for(n.__e=h,i=k;i--;)y[i]!=null&&(typeof n.type=="function"&&y[i].__e!=null&&y[i].__e==n.__d&&(n.__d=M(r,i+1)),_t(y[i],y[i]));if(v)for(i=0;i<v.length;i++)it(v[i],v[++i],v[++i])}function ot(t,e,n){var r,_;for(r=0;r<t.__k.length;r++)(_=t.__k[r])&&(_.__=t,e=typeof _.type=="function"?ot(_,e,n):rt(n,_,_,t.__k,_.__e,e));return e}function rt(t,e,n,r,_,o){var a,f,d;if(e.__d!==void 0)a=e.__d,e.__d=void 0;else if(n==null||_!=o||_.parentNode==null)t:if(o==null||o.parentNode!==t)t.appendChild(_),a=null;else{for(f=o,d=0;(f=f.nextSibling)&&d<r.length;d+=2)if(f==_)break t;t.insertBefore(_,o),a=o}return a!==void 0?a:_.nextSibling}function xt(t,e,n,r,_){var o;for(o in n)o==="children"||o==="key"||o in e||D(t,o,null,n[o],r);for(o in e)_&&typeof e[o]!="function"||o==="children"||o==="key"||o==="value"||o==="checked"||n[o]===e[o]||D(t,o,e[o],n[o],r)}function st(t,e,n){e[0]==="-"?t.setProperty(e,n):t[e]=n==null?"":typeof n!="number"||wt.test(e)?n:n+"px"}function D(t,e,n,r,_){var o;t:if(e==="style")if(typeof n=="string")t.style.cssText=n;else{if(typeof r=="string"&&(t.style.cssText=r=""),r)for(e in r)n&&e in n||st(t.style,e,"");if(n)for(e in n)r&&n[e]===r[e]||st(t.style,e,n[e])}else if(e[0]==="o"&&e[1]==="n")o=e!==(e=e.replace(/Capture$/,"")),e=e.toLowerCase()in t?e.toLowerCase().slice(2):e.slice(2),t.l||(t.l={}),t.l[e+o]=n,n?r||t.addEventListener(e,o?ct:at,o):t.removeEventListener(e,o?ct:at,o);else if(e!=="dangerouslySetInnerHTML"){if(_)e=e.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(e!=="href"&&e!=="list"&&e!=="form"&&e!=="tabIndex"&&e!=="download"&&e in t)try{t[e]=n==null?"":n;break t}catch(a){}typeof n=="function"||(n!=null&&(n!==!1||e[0]==="a"&&e[1]==="r")?t.setAttribute(e,n):t.removeAttribute(e))}}function at(t){this.l[t.type+!1](c.event?c.event(t):t)}function ct(t){this.l[t.type+!0](c.event?c.event(t):t)}function q(t,e,n,r,_,o,a,f,d){var p,i,g,u,s,m,h,v,y,k,U,x=e.type;if(e.constructor!==void 0)return null;n.__h!=null&&(d=n.__h,f=e.__e=n.__e,e.__h=null,o=[f]),(p=c.__b)&&p(e);try{t:if(typeof x=="function"){if(v=e.props,y=(p=x.contextType)&&r[p.__c],k=p?y?y.props.value:p.__:r,n.__c?h=(i=e.__c=n.__c).__=i.__E:("prototype"in x&&x.prototype.render?e.__c=i=new x(v,k):(e.__c=i=new E(v,k),i.constructor=x,i.render=St),y&&y.sub(i),i.props=v,i.state||(i.state={}),i.context=k,i.__n=r,g=i.__d=!0,i.__h=[]),i.__s==null&&(i.__s=i.state),x.getDerivedStateFromProps!=null&&(i.__s==i.state&&(i.__s=C({},i.__s)),C(i.__s,x.getDerivedStateFromProps(v,i.__s))),u=i.props,s=i.state,g)x.getDerivedStateFromProps==null&&i.componentWillMount!=null&&i.componentWillMount(),i.componentDidMount!=null&&i.__h.push(i.componentDidMount);else{if(x.getDerivedStateFromProps==null&&v!==u&&i.componentWillReceiveProps!=null&&i.componentWillReceiveProps(v,k),!i.__e&&i.shouldComponentUpdate!=null&&i.shouldComponentUpdate(v,i.__s,k)===!1||e.__v===n.__v){i.props=v,i.state=i.__s,e.__v!==n.__v&&(i.__d=!1),i.__v=e,e.__e=n.__e,e.__k=n.__k,e.__k.forEach(function(P){P&&(P.__=e)}),i.__h.length&&a.push(i);break t}i.componentWillUpdate!=null&&i.componentWillUpdate(v,i.__s,k),i.componentDidUpdate!=null&&i.__h.push(function(){i.componentDidUpdate(u,s,m)})}i.context=k,i.props=v,i.state=i.__s,(p=c.__r)&&p(e),i.__d=!1,i.__v=e,i.__P=t,p=i.render(i.props,i.state,i.context),i.state=i.__s,i.getChildContext!=null&&(r=C(C({},r),i.getChildContext())),g||i.getSnapshotBeforeUpdate==null||(m=i.getSnapshotBeforeUpdate(u,s)),U=p!=null&&p.type===j&&p.key==null?p.props.children:p,lt(t,Array.isArray(U)?U:[U],e,n,r,_,o,a,f,d),i.base=e.__e,e.__h=null,i.__h.length&&a.push(i),h&&(i.__E=i.__=null),i.__e=!1}else o==null&&e.__v===n.__v?(e.__k=n.__k,e.__e=n.__e):e.__e=Ct(n.__e,e,n,r,_,o,a,d);(p=c.diffed)&&p(e)}catch(P){e.__v=null,(d||o!=null)&&(e.__e=f,e.__h=!!d,o[o.indexOf(f)]=null),c.__e(P,e,n)}}function nt(t,e){c.__c&&c.__c(e,t),t.some(function(n){try{t=n.__h,n.__h=[],t.some(function(r){r.call(n)})}catch(r){c.__e(r,n.__v)}})}function Ct(t,e,n,r,_,o,a,f){var d,p,i,g,u=n.props,s=e.props,m=e.type,h=0;if(m==="svg"&&(_=!0),o!=null){for(;h<o.length;h++)if((d=o[h])&&(d===t||(m?d.localName==m:d.nodeType==3))){t=d,o[h]=null;break}}if(t==null){if(m===null)return document.createTextNode(s);t=_?document.createElementNS("http://www.w3.org/2000/svg",m):document.createElement(m,s.is&&s),o=null,f=!1}if(m===null)u===s||f&&t.data===s||(t.data=s);else{if(o=o&&R.slice.call(t.childNodes),p=(u=n.props||A).dangerouslySetInnerHTML,i=s.dangerouslySetInnerHTML,!f){if(o!=null)for(u={},g=0;g<t.attributes.length;g++)u[t.attributes[g].name]=t.attributes[g].value;(i||p)&&(i&&(p&&i.__html==p.__html||i.__html===t.innerHTML)||(t.innerHTML=i&&i.__html||""))}if(xt(t,s,u,_,f),i)e.__k=[];else if(h=e.props.children,lt(t,Array.isArray(h)?h:[h],e,n,r,_&&m!=="foreignObject",o,a,t.firstChild,f),o!=null)for(h=o.length;h--;)o[h]!=null&&Z(o[h]);f||("value"in s&&(h=s.value)!==void 0&&(h!==t.value||m==="progress"&&!h)&&D(t,"value",h,u.value,!1),"checked"in s&&(h=s.checked)!==void 0&&h!==t.checked&&D(t,"checked",h,u.checked,!1))}return t}function it(t,e,n){try{typeof t=="function"?t(e):t.current=e}catch(r){c.__e(r,n)}}function _t(t,e,n){var r,_,o;if(c.unmount&&c.unmount(t),(r=t.ref)&&(r.current&&r.current!==t.__e||it(r,null,e)),n||typeof t.type=="function"||(n=(_=t.__e)!=null),t.__e=t.__d=void 0,(r=t.__c)!=null){if(r.componentWillUnmount)try{r.componentWillUnmount()}catch(a){c.__e(a,e)}r.base=r.__P=null}if(r=t.__k)for(o=0;o<r.length;o++)r[o]&&_t(r[o],e,n);_!=null&&Z(_)}function St(t,e,n){return this.constructor(t,n)}function F(t,e,n){var r,_,o;c.__&&c.__(t,e),_=(r=typeof n=="function")?null:n&&n.__k||e.__k,o=[],q(e,t=(!r&&n||e).__k=l(j,null,[t]),_||A,A,e.ownerSVGElement!==void 0,!r&&n?[n]:_?null:e.firstChild?R.slice.call(e.childNodes):null,o,!r&&n?n:_?_.__e:e.firstChild,r),nt(o,t)}c={__e:function(t,e){for(var n,r,_;e=e.__;)if((n=e.__c)&&!n.__)try{if((r=n.constructor)&&r.getDerivedStateFromError!=null&&(n.setState(r.getDerivedStateFromError(t)),_=n.__d),n.componentDidCatch!=null&&(n.componentDidCatch(t),_=n.__d),_)return n.__E=n}catch(o){t=o}throw t},__v:0},E.prototype.setState=function(t,e){var n;n=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=C({},this.state),typeof t=="function"&&(t=t(C({},n),this.props)),t&&C(n,t),t!=null&&this.__v&&(e&&this.__h.push(e),et(this))},E.prototype.forceUpdate=function(t){this.__v&&(this.__e=!0,t&&this.__h.push(t),et(this))},E.prototype.render=j,N=[],Y=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,$.__r=0;var ut,w,ft,z=0,V=[],pt=c.__b,dt=c.__r,ht=c.diffed,mt=c.__c,vt=c.unmount;function jt(t,e){c.__h&&c.__h(w,t,z||e),z=0;var n=w.__H||(w.__H={__:[],__h:[]});return t>=n.__.length&&n.__.push({}),n.__[t]}function b(t){return z=1,Et(gt,t)}function Et(t,e,n){var r=jt(ut++,2);return r.t=t,r.__c||(r.__=[n?n(e):gt(void 0,e),function(_){var o=r.t(r.__[0],_);r.__[0]!==o&&(r.__=[o,r.__[1]],r.__c.setState({}))}],r.__c=w),r.__}function Nt(){V.forEach(function(t){if(t.__P)try{t.__H.__h.forEach(L),t.__H.__h.forEach(W),t.__H.__h=[]}catch(e){t.__H.__h=[],c.__e(e,t.__v)}}),V=[]}c.__b=function(t){w=null,pt&&pt(t)},c.__r=function(t){dt&&dt(t),ut=0;var e=(w=t.__c).__H;e&&(e.__h.forEach(L),e.__h.forEach(W),e.__h=[])},c.diffed=function(t){ht&&ht(t);var e=t.__c;e&&e.__H&&e.__H.__h.length&&(V.push(e)!==1&&ft===c.requestAnimationFrame||((ft=c.requestAnimationFrame)||function(n){var r,_=function(){clearTimeout(o),yt&&cancelAnimationFrame(r),setTimeout(n)},o=setTimeout(_,100);yt&&(r=requestAnimationFrame(_))})(Nt)),w=void 0},c.__c=function(t,e){e.some(function(n){try{n.__h.forEach(L),n.__h=n.__h.filter(function(r){return!r.__||W(r)})}catch(r){e.some(function(_){_.__h&&(_.__h=[])}),e=[],c.__e(r,n.__v)}}),mt&&mt(t,e)},c.unmount=function(t){vt&&vt(t);var e=t.__c;if(e&&e.__H)try{e.__H.__.forEach(L)}catch(n){c.__e(n,e.__v)}};var yt=typeof requestAnimationFrame=="function";function L(t){var e=w;typeof t.__c=="function"&&t.__c(),w=e}function W(t){var e=w;t.__c=t.__(),w=e}function gt(t,e){return typeof e=="function"?e(t):e}async function Mt(t,e){let n=window.__xsrf_token;return await(await fetch("/graphql",{method:"POST",credentials:"include",headers:{"Content-Type":"application/json","X-XSRF-Token":n},body:JSON.stringify({query:t,variables:e})})).json()}function I([t]){return t}function H(t,{onCommit:e,onError:n}){let[r,_]=b(!1),[o,a]=b(void 0);if(o!==void 0)throw o;return{commit:d=>{_(!0),Mt(t,d).then(({data:p,errors:i})=>{_(!1),i!=null?typeof n=="function"?n(i):a(i):e(p)}).catch(p=>a(p))},inFlight:r}}function G({size:t="small",style:e=""}){return l("svg",{xmlns:"http://www.w3.org/2000/svg",class:`${t==="large"?"h-8 w-8":"h-4 w-4"} ${e!=null?e:""}`,viewBox:"0 0 24 24",stroke:"none",fill:"currentColor"},l("rect",{x:"10",y:"0",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 6.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.125s",repeatCount:"indefinite"})),l("rect",{x:"16",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.25s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 17.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.375s",repeatCount:"indefinite"})),l("rect",{x:"10",y:"16",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.5s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 17.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.625s",repeatCount:"indefinite"})),l("rect",{x:"0",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.75s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 6.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.875s",repeatCount:"indefinite"})))}function S({title:t,onClick:e,disabled:n=!1,loading:r=!1,type:_="default",style:o}){let a=`h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${o!=null?o:""}`;switch(_){case"flat":a+=" bg-white text-black",n||(a+=" hover:bg-gray-200");break;case"default":default:a+=" bg-blue-500 text-white",n||(a+=" hover:bg-blue-400")}return n===!0&&(a+=" opacity-80 cursor-not-allowed"),l("button",{class:a,onClick:n===!0?void 0:e},l("div",null,t),r===!0&&l(G,null))}function It(){window.location.href=window.location.href}function Ht(t){let e={};for(let n of Object.getOwnPropertyNames(t))e[n]=t[n];return JSON.stringify(e)}function Tt({error:t}){return l("div",{class:"w-full flex justify-center p-8"},l("div",{class:"w-full max-w-3xl bg-red-500 rounded-lg p-4"},l("h1",{class:"text-white text-xl font-semibold"},"Error"),l("p",{class:"text-red-100"},"Sorry, something went wrong."),l("pre",{class:"text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap break-all"},Ht(t)),l(S,{title:"Reload Page",onClick:It,type:"flat",style:"w-full mt-2"})))}var J=class extends E{constructor(){super();this.state={error:!1}}componentDidCatch(e){console.error(e),this.setState({error:e})}render({children:e},{error:n}){return n!==!1?l(Tt,{error:n}):e}},bt=J;function T({title:t,href:e,disabled:n=!1,style:r}){let _=`
    block h-8 px-2
    flex space-x-2 justify-center items-center
    rounded-md font-semibold ${r!=null?r:""}
    text-blue-500 dark:text-blue-300
    hover:bg-gray-200 dark:hover:bg-gray-600
  `;return n===!0&&(_+=" opacity-80 cursor-not-allowed"),l("a",{class:_,href:e},t)}function Q({value:t,onChange:e,label:n,placeholder:r,style:_}){let[o]=b(`input-id-${Math.floor(Math.random()*1e4)}`);return l("div",{class:`w-full ${_!=null?_:""}`},l("label",{class:"text-gray-500 italic",for:o},n),l("input",{class:"w-full p-2 text-md rounded-md bg-gray-200 dark:bg-gray-600 text-black dark:text-white",id:o,type:"text",value:t,onInput:a=>typeof e=="function"&&e(a.target.value),placeholder:r}))}var Ut=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"})),kt=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}));function O({type:t="info",message:e,style:n}){let r=`flex align-start p-3 rounded-md block font-semibold ${n!=null?n:""}`,_;switch(t){case"error":r+=" bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",_=kt;break;case"warning":r+=" bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",_=kt;break;case"info":default:r+=" bg-blue-200 text-blue-800 dark:bg-blue-800 dark:text-blue-200",_=Ut}return l("div",{class:r},l("div",{class:"mr-2"},_),l("p",{class:"leading-snug"},e))}function X(){let[t,e]=b(""),[n,r]=b(null),[_,o]=b(null),{commit:a,inFlight:f}=H(I`
    mutation SubmitUrl($url: String!) {
      submitUrl(input: { url: $url }) {
        id
        title
      }
    }
  `,{onCommit:({submitUrl:{id:p,title:i}})=>{e(""),o(null),r(i?`"${i}" was successfully submitted`:"Your url was successfully submitted")},onError:([{message:p}])=>{r(null),o(`Failed to submit: ${p}`)}});return l("form",{class:"w-full",onSubmit:p=>{p.preventDefault(),a({url:t.trim()})}},n&&l(O,{message:n,style:"mb-2"}),_&&l(O,{message:_,type:"error",style:"mb-2"}),l(Q,{label:"URL to something interesting",placeholder:"https://urls.fyi",value:t,onChange:e}),l(S,{title:"Submit",style:"w-full mt-2",loading:f,disabled:f}))}var Pt=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-5 w-5",viewBox:"0 0 20 20",fill:"currentColor"},l("path",{"fill-rule":"evenodd",d:"M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM9 15a1 1 0 011-1h6a1 1 0 110 2h-6a1 1 0 01-1-1z","clip-rule":"evenodd"}));function At(){let[t,e]=b(!1),[n,r]=b(!1),_="w-full rounded-t-lg pb-8 -mb-6 p-2";(t||n)&&(_+=" bg-gray-50 dark:bg-gray-900");let o=l(j,null,l(T,{title:"home",href:"/"}),l(T,{title:"recent",href:"/recent"}),l(T,{title:"mine",href:"/mine"}));return l("div",{class:_},l("div",{class:"flex items-center justify-between"},l(S,{title:"Submit url",onClick:()=>{e(!(t||n)),r(!1)},style:"mr-2 whitespace-nowrap"}),l("div",{class:"w-full sm:flex hidden"},o),l(S,{title:Pt,onClick:()=>{r(!(t||n)),e(!1)},type:"flat",style:"sm:hidden block"})),l("div",{class:"mt-4",style:{display:t?void 0:"none"}},l(X,null)),n&&l("div",{class:"mt-4"},o))}function Bt({urlID:t,initDidVote:e,initCount:n}){let[r,_]=b(e!=null?e:!1),[o,a]=b(n!=null?n:0),f=`
        block w-10 h-10 flex-none m-2 sm:mx-0
        flex flex-col justify-center items-center
        border-2 border-blue-500 rounded
        ${r?"bg-blue-500 text-white":"text-blue-500"}
  `,d=({url:u})=>{var s,m;a((s=u==null?void 0:u.upvoteCount)!=null?s:o),_((m=u==null?void 0:u.upvotedByViewer)!=null?m:r)},p=H(I`
    mutation Upvote($id: ID!) {
      url: upvoteUrl(url: $id) {
        id
        upvotedByViewer
        upvoteCount
      }
    }
  `,{onCommit:d,onError:()=>{}}),i=H(I`
    mutation Upvote($id: ID!) {
      url: rescindUrlUpvote(url: $id) {
        id
        upvotedByViewer
        upvoteCount
      }
    }
  `,{onCommit:d,onError:()=>{}});return l("button",{class:f,onClick:u=>{u.preventDefault(),r?(a(o-1),_(!1),i.commit({id:t})):(a(o+1),_(!0),p.commit({id:t}))}},l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-5 w-5",viewBox:"0 0 20 20",fill:"currentColor"},l("path",{"fill-rule":"evenodd",d:"M5.293 9.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 7.414V15a1 1 0 11-2 0V7.414L6.707 9.707a1 1 0 01-1.414 0z","clip-rule":"evenodd"})),l("span",{class:"text-xs leading-none"},o))}F(l(bt,null,l(At,null)),document.getElementById("header"));for(let t of document.querySelectorAll("[data-hydrate-vote-button]")){let e=t.dataset.id,n=parseInt(t.dataset.count),r=t.dataset.upvoted==="true";F(l(Bt,{urlID:e,initDidVote:r,initCount:n}),t)}
//# sourceMappingURL=page.js.map
