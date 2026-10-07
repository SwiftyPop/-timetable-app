/* alive.js — extra motion layer for the timetable app.
   Self-contained: injects its own CSS, reads the app's globals (nowInfo, S, DF, render…) when present.
   Only transform/opacity are animated (plus one registered colour/angle var). Everything honours prefers-reduced-motion. */
(function(){
'use strict';
var mq=window.matchMedia?matchMedia('(prefers-reduced-motion: reduce)'):{matches:false};
function still(){
  if(typeof isReducedMotion==='function')return isReducedMotion();
  return mq.matches;
}
var fine=!!(window.matchMedia&&matchMedia('(hover:hover) and (pointer:fine)').matches);
function $(i){return document.getElementById(i)}
function gone(el){return function(){el.remove()}}

var css='\
@property --ah{syntax:"<number>";inherits:true;initial-value:150}\
@property --ang{syntax:"<angle>";inherits:false;initial-value:0deg}\
:root{--ah:150;transition:--ah 2.4s ease}\
body{overscroll-behavior-y:contain}\
.al-blobs{position:fixed;inset:0;z-index:-1;pointer-events:none;overflow:hidden}\
.al-blobs i{position:absolute;width:52vmax;height:52vmax;border-radius:50%;opacity:.2;will-change:transform}\
.al-blobs i:nth-child(1){left:-14vmax;top:-12vmax;background:radial-gradient(circle,hsl(var(--ah) 85% 60%),transparent 68%);animation:alD1 28s ease-in-out infinite alternate}\
.al-blobs i:nth-child(2){right:-16vmax;top:28vh;background:radial-gradient(circle,hsl(calc(var(--ah) + 70) 80% 62%),transparent 68%);animation:alD2 34s ease-in-out infinite alternate}\
.al-blobs i:nth-child(3){left:18vw;bottom:-22vmax;background:radial-gradient(circle,hsl(calc(var(--ah) - 60) 80% 60%),transparent 68%);animation:alD3 40s ease-in-out infinite alternate}\
@keyframes alD1{to{transform:translate3d(16vw,12vh,0) scale(1.18)}}\
@keyframes alD2{to{transform:translate3d(-18vw,-10vh,0) scale(.88)}}\
@keyframes alD3{to{transform:translate3d(12vw,-14vh,0) scale(1.12)}}\
.al-ch{display:inline-block;opacity:0;transform:translateY(.55em) rotate(7deg);animation:alCh .65s cubic-bezier(.2,1.3,.3,1) forwards;animation-delay:calc(var(--c)*42ms + 60ms)}\
@keyframes alCh{to{opacity:1;transform:none}}\
.al-pop{animation:alPop .55s cubic-bezier(.2,1.5,.4,1) backwards;animation-delay:calc(var(--s)*80ms + 380ms)}\
@keyframes alPop{from{opacity:0;transform:scale(.6) translateY(10px)}}\
.al-greet{display:flex;align-items:center;gap:12px;margin:-4px 0 14px}\
.al-greet svg{flex:none;overflow:visible;color:var(--ink)}\
.al-greet b{display:block;font-size:15px;font-weight:700}\
.al-greet span{display:block;font-size:12.5px;color:var(--mute);margin-top:1px}\
.al-sun{transition:transform 1.4s cubic-bezier(.3,.8,.2,1)}\
.al-sun text{font-size:12px;text-anchor:middle;dominant-baseline:central;transform-box:fill-box;transform-origin:center;animation:alBob 3.2s ease-in-out infinite}\
@keyframes alBob{50%{transform:translateY(-2px) rotate(8deg)}}\
.al-swap{animation:alSwap .45s cubic-bezier(.2,1,.3,1)}\
@keyframes alSwap{from{opacity:0;transform:translateY(6px)}}\
@media(hover:hover) and (pointer:fine){\
.ev:hover,.blk:hover{background-image:radial-gradient(240px circle at var(--mx,50%) var(--my,50%),hsl(var(--h) 95% 72% / .2),transparent 70%)}\
.ev{transition:transform .25s cubic-bezier(.2,.8,.2,1),box-shadow .25s}.ev.al-t{transition:transform .06s linear,box-shadow .25s}}\
@supports (background:conic-gradient(from 0deg,red,blue)){\
.timeline-item.live .ev{position:relative}\
.timeline-item.live .ev::before{content:"";position:absolute;inset:-2px;border-radius:inherit;padding:2px;pointer-events:none;background:conic-gradient(from var(--ang),transparent 0 55%,hsl(var(--h) 92% 64%) 85%,transparent 100%);-webkit-mask:linear-gradient(#000 0 0) content-box,linear-gradient(#000 0 0);-webkit-mask-composite:xor;mask:linear-gradient(#000 0 0) content-box,linear-gradient(#000 0 0);mask-composite:exclude;animation:alSpin 3.4s linear infinite}}\
@keyframes alSpin{to{--ang:360deg}}\
.now.al-ending .clk b{transform-origin:left center;animation:alBeat 1.1s ease-in-out infinite}\
@keyframes alBeat{0%,100%{transform:scale(1)}14%{transform:scale(1.07)}28%{transform:scale(1)}42%{transform:scale(1.04)}}\
.al-spark{position:fixed;z-index:9999;width:6px;height:6px;border-radius:50%;pointer-events:none;background:hsl(var(--ah) 90% 62%)}\
.al-conf{position:fixed;top:0;z-index:9998;pointer-events:none;border-radius:2px}\
.al-in{animation:alIn .5s cubic-bezier(.2,1,.3,1) backwards;animation-delay:calc(var(--k)*45ms + 120ms)}\
@keyframes alIn{from{opacity:0;transform:translateY(14px)}}\
.al-jump{position:fixed;left:50%;bottom:calc(92px + env(safe-area-inset-bottom,0px));z-index:60;border:0;border-radius:99px;padding:10px 15px;background:var(--ac);color:var(--acink);font:700 12.5px/1 inherit;font-family:inherit;box-shadow:0 8px 24px -6px rgba(0,0,0,.45);opacity:0;pointer-events:none;transform:translate(-50%,24px) scale(.9);transition:transform .35s cubic-bezier(.2,1.4,.4,1),opacity .25s;cursor:pointer}\
.al-jump.on{opacity:1;pointer-events:auto;transform:translate(-50%,0) scale(1)}\
.al-jump i{display:inline-block;font-style:normal;margin-right:5px;animation:alArrow 1s ease-in-out infinite}\
@keyframes alArrow{50%{transform:translateY(3px)}}\
.al-ptr{position:fixed;top:max(8px,env(safe-area-inset-top,8px));left:50%;width:34px;height:34px;margin-left:-17px;border-radius:50%;background:var(--card);border:1px solid var(--line);display:grid;place-items:center;font-size:17px;z-index:70;opacity:0;transform:translateY(-50px);pointer-events:none;box-shadow:0 6px 18px -6px rgba(0,0,0,.35)}\
.al-sp{position:fixed;top:0;left:0;right:0;height:3px;z-index:80;transform-origin:left;transform:scaleX(0);background:linear-gradient(90deg,hsl(var(--ah) 90% 60%),hsl(calc(var(--ah) + 60) 90% 62%));pointer-events:none}\
.al-hidden .al-blobs i,.al-hidden .timeline-item.live .ev::before{animation-play-state:paused}\
@media print{.al-blobs,.al-jump,.al-ptr,.al-sp,.al-greet{display:none!important}}\
@media(prefers-reduced-motion:reduce){:root:not(.force-motion) .al-blobs i,:root:not(.force-motion) .al-sun text,:root:not(.force-motion) .timeline-item.live .ev::before,:root:not(.force-motion) .now.al-ending .clk b,:root:not(.force-motion) .al-jump i{animation:none!important}:root:not(.force-motion) .al-ch,:root:not(.force-motion) .al-pop,:root:not(.force-motion) .al-swap,:root:not(.force-motion) .al-in{animation:none!important;opacity:1!important;transform:none!important}:root:not(.force-motion) .al-sp{display:none}:root:not(.force-motion) .al-blobs{display:none!important}}\
html.reduce-motion .al-blobs i,html.reduce-motion .al-sun text,html.reduce-motion .timeline-item.live .ev::before,html.reduce-motion .now.al-ending .clk b,html.reduce-motion .al-jump i{animation:none!important}\
html.reduce-motion .al-ch,html.reduce-motion .al-pop,html.reduce-motion .al-swap,html.reduce-motion .al-in{animation:none!important;opacity:1!important;transform:none!important}\
html.reduce-motion .al-sp{display:none}\
html.reduce-motion .al-blobs{display:none!important}';
var st=document.createElement('style');st.textContent=css;document.head.appendChild(st);

/* ambient blobs, tinted by the current subject */
var bl=document.createElement('div');bl.className='al-blobs';bl.setAttribute('aria-hidden','true');bl.innerHTML='<i></i><i></i><i></i>';if(still())bl.style.display='none';document.body.appendChild(bl);
document.addEventListener('visibilitychange',function(){document.documentElement.classList.toggle('al-hidden',document.hidden)});
window.addEventListener('motionchange',function(){if(bl)bl.style.display=still()?'none':'';if(still()){var j=document.querySelector('.al-jump');if(j)j.classList.remove('on')}pulse()});

/* intro: letters spring in, pills pop, numbers count up */
(function(){
  var h=document.querySelector('h1');
  if(h&&!h.dataset.al&&!still()){h.dataset.al=1;var t=h.textContent;h.setAttribute('aria-label',t);h.textContent='';
    for(var i=0;i<t.length;i++){var s=document.createElement('span');s.className='al-ch';s.setAttribute('aria-hidden','true');s.style.setProperty('--c',i);s.textContent=t[i]===' '?'\u00a0':t[i];h.appendChild(s)}}
  if(still())return;
  [].forEach.call(document.querySelectorAll('#stats .stat'),function(e,i){e.style.setProperty('--s',i);e.classList.add('al-pop');e.addEventListener('animationend',function(){e.classList.remove('al-pop')},{once:true})});
  [].forEach.call(document.querySelectorAll('#stats b'),function(b){
    var raw=b.textContent.trim(),v=parseInt(raw,10);if(!/^\d+$/.test(raw)||!(v>0))return;
    var t0=performance.now();(function f(t){var k=Math.min(1,(t-t0)/900),e=1-Math.pow(1-k,3);b.textContent=Math.round(v*e);if(k<1)requestAnimationFrame(f)})(t0)});
})();

/* greeting + sun/moon arc + live status line */
var G=null,gt='',gs='';
function dur(m){m=Math.max(1,Math.ceil(m));if(m<60)return m+' min';var h=Math.floor(m/60),r=m%60;if(h>=24)return Math.floor(h/24)+' d '+(h%24)+' h';return h+' h'+(r?' '+r+' min':'')}
function greetBuild(){
  var h=document.querySelector('h1');if(!h||G)return;
  G=document.createElement('div');G.className='al-greet';
  G.innerHTML='<svg width="68" height="38" viewBox="0 0 80 44" aria-hidden="true"><path d="M6 40A34 34 0 0 1 74 40" fill="none" stroke="currentColor" stroke-opacity=".25" stroke-width="2" stroke-dasharray="2 5" stroke-linecap="round"/><g class="al-sun"><text></text></g></svg><div><b></b><span></span></div>';
  h.insertAdjacentElement('afterend',G);
}
function swap(el,txt){if(el.textContent===txt)return;el.textContent=txt;if(still())return;el.classList.remove('al-swap');void el.offsetWidth;el.classList.add('al-swap')}
var prev=null,doneFor='';
function pulse(){
  if(typeof nowInfo!=='function'||typeof S==='undefined')return;
  var I=nowInfo(),m=I.m,H=Math.floor(m/60);
  if(G){
    var day=m>=360&&m<1200,f=day?(m-360)/840:((m>=1200?m-1200:m+240)/600);
    var x=40-34*Math.cos(Math.PI*f),y=40-34*Math.sin(Math.PI*f),tx=G.querySelector('text'),ico=day?(H<17?'☀️':'🌇'):'🌙';
    G.querySelector('.al-sun').style.transform='translate('+x.toFixed(1)+'px,'+y.toFixed(1)+'px)';
    if(tx.textContent!==ico)tx.textContent=ico;
    var g=H<5?'Burning the midnight oil':H<12?'Good morning':H<17?'Good afternoon':H<20?'Good evening':'Good night',s;
    if(I.cur)s=S[I.cur[3]].n+' · '+dur(I.cur[2]*60-m)+' left';
    else if(I.nxt){var mins=I.nd*1440+I.nxt[1]*60-m;s=(I.td&&I.nd>0?'That’s a wrap for today 🎉 · next class in ':I.nd===0?'Free · next class in ':'Next class in ')+dur(mins)}
    else s='No classes scheduled';
    swap(G.querySelector('b'),g);swap(G.querySelector('span'),s);
  }
  var hue=I.cur?S[I.cur[3]].h:(I.nxt?S[I.nxt[3]].h:150);
  document.documentElement.style.setProperty('--ah',hue);
  var n=$('now');if(n){var ending=!!(I.cur&&I.cur[2]*60-m<=5);if(n.classList.contains('al-ending')!==ending)n.classList.toggle('al-ending',ending)}
  var key=new Date().toDateString();
  if(prev&&prev.cur&&!I.cur&&I.td&&(!I.nxt||I.nd>0)&&doneFor!==key){doneFor=key;celebrate();try{if(typeof showToast==='function')showToast('That’s all for today. Nice work!','🎉')}catch(e){}}
  prev=I;
}
greetBuild();pulse();setInterval(pulse,20000);
document.addEventListener('visibilitychange',function(){if(!document.hidden)pulse()});
var nowEl=$('now');if(nowEl)new MutationObserver(function(){pulse()}).observe(nowEl,{attributes:true,attributeFilter:['class']});

/* spotlight + tilt on hover (mouse only) */
if(fine&&!still()){
  var lastT=null,raf=0,pe=null;
  document.addEventListener('pointermove',function(e){
    pe=e;if(raf)return;
    raf=requestAnimationFrame(function(){raf=0;
      var t=pe.target.closest?pe.target.closest('.ev,.blk'):null;
      if(lastT&&lastT!==t){lastT.style.transform='';lastT.classList.remove('al-t')}
      lastT=t;if(!t)return;
      var r=t.getBoundingClientRect(),px=(pe.clientX-r.left)/r.width,py=(pe.clientY-r.top)/r.height;
      t.style.setProperty('--mx',(px*100).toFixed(1)+'%');t.style.setProperty('--my',(py*100).toFixed(1)+'%');
      if(t.classList.contains('ev')){t.classList.add('al-t');t.style.transform='perspective(800px) rotateX('+((0.5-py)*5).toFixed(2)+'deg) rotateY('+((px-0.5)*7).toFixed(2)+'deg) scale(1.01)'}
    })},{passive:true});
  document.addEventListener('pointerleave',function(){if(lastT){lastT.style.transform='';lastT.classList.remove('al-t');lastT=null}},true);
}

/* spark burst on buttons + dock icon pop */
function spark(x,y){
  for(var i=0;i<8;i++){(function(i){
    var d=document.createElement('i');d.className='al-spark';d.style.left=x+'px';d.style.top=y+'px';document.body.appendChild(d);
    var a=Math.PI*2*i/8+Math.random()*.5,r=18+Math.random()*22;
    d.animate([{transform:'translate(-50%,-50%) scale(1)',opacity:1},{transform:'translate(calc(-50% + '+(Math.cos(a)*r).toFixed(1)+'px),calc(-50% + '+(Math.sin(a)*r).toFixed(1)+'px)) scale(0)',opacity:0}],{duration:420+Math.random()*180,easing:'cubic-bezier(.2,.8,.3,1)'}).onfinish=gone(d)})(i)}
}
document.addEventListener('pointerdown',function(e){
  if(still()||!e.target.closest)return;
  var b=e.target.closest('.pb,.mob-dock button,.tabs button,.seg button,.wp-close');if(!b)return;
  var ic=b.querySelector&&b.querySelector('.dock-icon');
  if(ic&&ic.animate)ic.animate([{transform:'scale(1)'},{transform:'scale(1.38) rotate(-9deg)'},{transform:'scale(1.1)'}],{duration:380,easing:'cubic-bezier(.2,1.6,.4,1)'});
  spark(e.clientX,e.clientY)},{passive:true});

/* confetti for "last class done" */
function celebrate(){
  if(still())return;var cols=['#ffd166','#ef476f','#06d6a0','#118ab2','#c77dff'];
  for(var i=0;i<38;i++){(function(i){
    var d=document.createElement('i');d.className='al-conf';
    d.style.cssText='left:'+(Math.random()*100).toFixed(1)+'vw;background:'+cols[i%5]+';width:'+(6+Math.random()*6).toFixed(0)+'px;height:'+(8+Math.random()*8).toFixed(0)+'px';
    document.body.appendChild(d);
    d.animate([{transform:'translate(0,-20px) rotate(0deg)',opacity:1},{transform:'translate('+((Math.random()-.5)*160).toFixed(0)+'px,105vh) rotate('+(Math.random()*720-360).toFixed(0)+'deg)',opacity:.9}],{duration:1800+Math.random()*1600,delay:Math.random()*500,easing:'cubic-bezier(.3,.1,.6,1)',fill:'both'}).onfinish=gone(d)})(i)}
}

/* staggered reveal when sheets open */
function stagger(root){
  if(still())return;
  [].forEach.call(root.querySelectorAll('.set-row,.wp-head,.wpc>*,.bottom-sheet h2,.bottom-sheet h3,.bottom-sheet p,.bottom-sheet button'),function(e,i){
    e.style.setProperty('--k',Math.min(i,12));e.classList.remove('al-in');void e.offsetWidth;e.classList.add('al-in')});
}
var mo=new MutationObserver(function(ms){ms.forEach(function(m){
  var t=m.target;if(!t.classList.contains('open')||(m.oldValue&&/\bopen\b/.test(m.oldValue)))return;stagger(t)})});
[].forEach.call(document.querySelectorAll('.android-expand,.bottom-sheet'),function(e){mo.observe(e,{attributes:true,attributeFilter:['class'],attributeOldValue:true})});

/* jump-to-now pill + scroll progress */
var jp=document.createElement('button');jp.className='al-jump';jp.innerHTML='<i>↓</i>Jump to now';document.body.appendChild(jp);
var sp=document.createElement('div');sp.className='al-sp';sp.setAttribute('aria-hidden','true');document.body.appendChild(sp);
function liveEl(){return document.querySelector('.timeline-item.live,.gap-card.live,.day-nowm')}
var sraf=0;
function scrolled(){
  if(sraf)return;sraf=requestAnimationFrame(function(){sraf=0;
    var el=liveEl(),de=document.documentElement,max=de.scrollHeight-innerHeight;
    sp.style.transform='scaleX('+(max>0?Math.min(1,scrollY/max):0).toFixed(3)+')';
    if(!el){jp.classList.remove('on');return}
    var r=el.getBoundingClientRect(),below=r.top>innerHeight,off=r.bottom<0||below;
    jp.classList.toggle('on',off);jp.firstChild.textContent=below?'↓':'↑'})}
addEventListener('scroll',scrolled,{passive:true});addEventListener('resize',scrolled);
var dayEl=$('day');if(dayEl)new MutationObserver(scrolled).observe(dayEl,{childList:true});
jp.onclick=function(){var el=liveEl();if(el)el.scrollIntoView({behavior:still()?'auto':'smooth',block:'center'})};
setInterval(scrolled,5000);scrolled();

/* pull to refresh (touch): replays the day entrance */
if('ontouchstart' in window){
  var pt=document.createElement('div');pt.className='al-ptr';pt.textContent='↻';document.body.appendChild(pt);
  var y0=0,pull=0,on=false;
  addEventListener('touchstart',function(e){
    on=scrollY<=0&&e.touches.length===1&&!document.querySelector('.bottom-sheet.open,.android-expand.open');
    if(on){y0=e.touches[0].clientY;pull=0;pt.style.transition='none'}},{passive:true});
  addEventListener('touchmove',function(e){
    if(!on)return;var d=e.touches[0].clientY-y0;
    if(d<=0){pull=0;pt.style.opacity=0;return}
    pull=Math.min(110,d*.5);pt.style.opacity=Math.min(1,pull/50);pt.style.transform='translateY('+(pull-40)+'px) rotate('+(pull*4)+'deg)'},{passive:true});
  addEventListener('touchend',function(){
    if(!on)return;on=false;pt.style.transition='transform .35s cubic-bezier(.2,1,.3,1),opacity .3s';
    if(pull>=60){
      pt.style.transform='translateY(20px) rotate(360deg)';try{navigator.vibrate&&navigator.vibrate(15)}catch(e){}
      try{if(typeof lastDayRenderedHtml!=='undefined')lastDayRenderedHtml='';if(typeof render==='function')render();if(typeof tick==='function')tick(true)}catch(e){}
      pulse();setTimeout(function(){pt.style.opacity=0;pt.style.transform='translateY(-50px)'},650);
    }else{pt.style.opacity=0;pt.style.transform='translateY(-50px)'}
    pull=0},{passive:true});
}

window.alive={celebrate:celebrate,spark:spark};
})();
