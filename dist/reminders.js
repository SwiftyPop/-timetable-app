/* Class reminders + Google Calendar sync. Loaded (defer) after the main script; uses its globals. */
(function(){
'use strict';
var TZ='Asia/Kuala_Lumpur',UNTIL='20270228T160000Z',SCOPE='https://www.googleapis.com/auth/calendar.app.created',KEY='tt_rem',FK='tt_rem_fired';
var COLORS={ctrl:'6',ect:'3',el2:'7',mi:'10',mt:'4'};
var cfg=(function(){var d={on:false,lead:10,brief:false,off:{},cid:'',cal:''};try{var j=JSON.parse(localStorage.getItem(KEY)||'{}');for(var k in j)d[k]=j[k]}catch(e){}return d})();
function save(){try{localStorage.setItem(KEY,JSON.stringify(cfg))}catch(e){}}
function $(i){return document.getElementById(i)}
function toast(m,i){if(typeof showToast==='function')showToast(m,i)}
function status(m){var el=$('rm-status');if(el)el.textContent=m||''}
function esc(s){return String(s).replace(/[&<>"]/g,function(c){return{'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]})}
function ymd(){return new Intl.DateTimeFormat('en-CA',{timeZone:TZ}).format(new Date())}
function perm(){return 'Notification' in window?Notification.permission:'unsupported'}

/* ---------- local notifications ---------- */
function show(title,body,tag){
  var o={body:body,tag:tag,icon:'icons/icon-192.png',badge:'icons/icon-192.png',data:{url:'./?v=d'}};
  if(typeof tauriInvoke==='function'){try{tauriInvoke('notify_class_start',{title:title,body:body}).catch(function(){});}catch(e){}}
  var direct=function(){try{new Notification(title,o)}catch(e){}};
  if(!('serviceWorker' in navigator))return direct();
  var t=new Promise(function(_,rej){setTimeout(rej,1500)});
  Promise.race([navigator.serviceWorker.ready,t]).then(function(r){return r.showNotification(title,o)}).catch(direct);
}
function fired(day){try{var f=JSON.parse(localStorage.getItem(FK)||'{}');if(f.d!==day)f={d:day,k:{}};return f}catch(e){return{d:day,k:{}}}}
function check(){
  if(!cfg.on||perm()!=='granted'||typeof E==='undefined')return;
  var T=getMalaysiaTime(),m=T.mTotal;if(T.day<1||T.day>5)return;
  var f=fired(ymd()),dirty=false;
  var today=E.filter(function(x){return x[0]===T.day&&!cfg.off[x[3]]}).sort(function(a,b){return a[1]-b[1]});
  today.forEach(function(x){
    var k='c'+E.indexOf(x),left=x[1]*60-m;
    if(f.k[k]||left>cfg.lead||left<=-5)return;
    f.k[k]=1;dirty=true;var s=S[x[3]];
    show(left>0?s.n+' in '+Math.ceil(left)+' min':s.n+' is starting now',x[4]+' · '+rmOf(x)+' · '+fmtTime(x[1],0)+' – '+fmtTime(x[2],0),'class-'+k);
  });
  if(cfg.brief&&!f.k.b&&m>=450&&m<780){
    var up=today.filter(function(x){return x[2]*60>m});
    if(up.length){f.k.b=1;dirty=true;var x=up[0];show('Today: '+today.length+' class'+(today.length>1?'es':''),'Next: '+S[x[3]].n+' at '+fmtTime(x[1],0)+' · '+rmOf(x),'brief')}
  }
  if(dirty)try{localStorage.setItem(FK,JSON.stringify(f))}catch(e){}
}

/* ---------- Google Calendar ---------- */
function loadGIS(){return new Promise(function(res,rej){
  if(window.google&&google.accounts&&google.accounts.oauth2)return res();
  var s=document.createElement('script');s.src='https://accounts.google.com/gsi/client';s.async=true;
  s.onload=res;s.onerror=function(){rej(new Error('Could not load Google sign-in (are you offline?)'))};document.head.appendChild(s)})}
function token(){return loadGIS().then(function(){return new Promise(function(res,rej){
  var tc=google.accounts.oauth2.initTokenClient({client_id:cfg.cid,scope:SCOPE,
    callback:function(r){r.error?rej(new Error(r.error)):res(r.access_token)},
    error_callback:function(e){rej(new Error(e&&e.type==='popup_closed'?'Sign-in was cancelled':'Sign-in failed'))}});
  tc.requestAccessToken({prompt:cfg.cal?'':'consent'})})})}
function api(t,method,path,body){
  return fetch('https://www.googleapis.com/calendar/v3'+path,{method:method,headers:{Authorization:'Bearer '+t,'Content-Type':'application/json'},body:body?JSON.stringify(body):undefined})
  .then(function(r){if(r.status===204)return null;return r.json().catch(function(){return{}}).then(function(j){if(!r.ok){var e=new Error((j.error&&j.error.message)||('HTTP '+r.status));e.status=r.status;throw e}return j})})}
function pad(n){return (n<10?'0':'')+n}
function firstDate(x){var T=getMalaysiaTime(),a=ymd().split('-').map(Number),diff=(x[0]-T.day+7)%7;if(diff===0&&T.mTotal>=x[2]*60)diff=7;return new Date(Date.UTC(a[0],a[1]-1,a[2]+diff)).toISOString().slice(0,10)}
function evId(x){return 'tt'+x[0]+'h'+x[1]+'h'+x[2]+x[3]}   /* base32hex-safe: a-v, 0-9 */
function resource(x){
  var s=S[x[3]],d=firstDate(x);
  return{id:evId(x),status:'confirmed',summary:s.n+' ('+x[4]+')',location:x[5],description:s.c+'\nLecturers: '+(x[6]||'—'),
    start:{dateTime:d+'T'+pad(x[1])+':00:00',timeZone:TZ},end:{dateTime:d+'T'+pad(x[2])+':00:00',timeZone:TZ},
    recurrence:['RRULE:FREQ=WEEKLY;UNTIL='+UNTIL],colorId:COLORS[x[3]]||'8',
    reminders:{useDefault:false,overrides:[{method:'popup',minutes:cfg.lead}]}};
}
async function sync(){
  if(!cfg.cid){status('Paste your Google OAuth client ID below first (see SETUP.md).');return}
  status('Connecting to Google…');
  try{
    var t=await token();
    if(cfg.cal){try{await api(t,'GET','/calendars/'+encodeURIComponent(cfg.cal))}catch(e){if(e.status===404||e.status===410)cfg.cal='';else throw e}}
    if(!cfg.cal){var c=await api(t,'POST','/calendars',{summary:'UniMAP Timetable',timeZone:TZ,description:'Synced from my timetable app'});cfg.cal=c.id;save()}
    var base='/calendars/'+encodeURIComponent(cfg.cal)+'/events',want={},i=0;
    for(var n=0;n<E.length;n++){
      var r=resource(E[n]);want[r.id]=1;status('Syncing '+(++i)+' / '+E.length+'…');
      try{await api(t,'POST',base,r)}catch(e){if(e.status===409)await api(t,'PUT',base+'/'+r.id,r);else throw e}
    }
    var list=await api(t,'GET',base+'?maxResults=250'),gone=0;
    for(var j=0;j<(list.items||[]).length;j++){var it=list.items[j];if(/^tt/.test(it.id)&&!want[it.id]){await api(t,'DELETE',base+'/'+it.id);gone++}}
    status('Synced '+E.length+' weekly classes'+(gone?' · removed '+gone+' old':'')+' to “UniMAP Timetable” ✓');toast('Google Calendar synced','📅');
  }catch(e){status('Sync failed: '+e.message);toast('Sync failed','⚠️')}
}
var delArm=0;
async function removeCal(){
  if(!cfg.cal){status('Nothing to remove yet.');return}
  if(!delArm){delArm=1;$('gc-del').textContent='Tap again to confirm';setTimeout(function(){delArm=0;$('gc-del').textContent='🗑 Remove'},4000);return}
  delArm=0;$('gc-del').textContent='🗑 Remove';status('Removing…');
  try{var t=await token();await api(t,'DELETE','/calendars/'+encodeURIComponent(cfg.cal));cfg.cal='';save();status('Removed the “UniMAP Timetable” calendar.')}catch(e){status('Could not remove: '+e.message)}
}
function subscribe(){
  var u=new URL('schedule.ics',location.href).href.replace(/^https?:/,'webcal:');
  window.open('https://calendar.google.com/calendar/r?cid='+encodeURIComponent(u),'_blank','noopener');
}

/* ---------- settings UI ---------- */
function ui(){
  var card=$('set-card');if(!card||$('rm-row'))return;
  var codes=Object.keys(S).map(function(k){return '<button data-k="'+k+'" class="'+(cfg.off[k]?'':'on')+'">'+S[k].c.split(' / ')[0]+'</button>'}).join('');
  var leads=[5,10,15,30].map(function(n){return '<button data-l="'+n+'" class="'+(cfg.lead===n?'on':'')+'">'+n+' min</button>'}).join('');
  var inp='font:inherit;font-size:13px;padding:8px 10px;border-radius:10px;border:1px solid var(--line);background:var(--card);color:var(--ink);width:100%';
  card.insertAdjacentHTML('beforeend',
   '<div id="rm-row" class="set-row"><div class="set-info"><b>Class reminders</b><span>Notify me before each class while the app is open or running in the background</span></div><div style="display:flex;gap:6px"><button class="pb" id="rm-on"></button><button class="pb" id="rm-test">🔔 Test</button></div></div>'+
   '<div class="set-row"><div class="set-info"><b>Remind me before</b><span>Also used for Google Calendar pop-ups</span></div><div class="seg" id="rm-lead">'+leads+'</div></div>'+
   '<div class="set-row"><div class="set-info"><b>Morning summary</b><span>One notification at 07:30 on class days</span></div><button class="pb" id="rm-brief"></button></div>'+
   '<div class="set-row"><div class="set-info"><b>Remind me for</b><span>Tap a subject to mute it</span></div><div class="seg" id="rm-subj" style="flex-wrap:wrap">'+codes+'</div></div>'+
   '<div class="set-row"><div class="set-info"><b>Google Calendar</b><span>Subscribe: read-only copy that updates itself. Sync: your own events with phone pop-up reminders, even when this app is closed.</span></div><div style="display:flex;gap:6px;flex-wrap:wrap"><button class="pb" id="gc-sub">➕ Subscribe</button><button class="pb" id="gc-sync">🔄 Sync</button><button class="pb" id="gc-del">🗑 Remove</button></div></div>'+
   '<div class="set-row" style="align-items:stretch"><div class="set-info" style="width:100%"><b>Google OAuth client ID</b><span>Needed once for Sync (see SETUP.md). Stored only in this browser.</span></div><input id="gc-cid" style="'+inp+'" placeholder="1234567890-abc.apps.googleusercontent.com" autocomplete="off" spellcheck="false" value="'+esc(cfg.cid)+'"></div>'+
   '<div class="note" id="rm-status" role="status" aria-live="polite" style="margin:8px 0 0"></div>');
  function paint(){var on=cfg.on&&perm()==='granted';$('rm-on').textContent=on?'✓ On':'Enable';$('rm-on').classList.toggle('on',on);$('rm-brief').textContent=cfg.brief?'✓ On':'Off';$('rm-brief').classList.toggle('on',cfg.brief)}
  paint();
  $('rm-on').onclick=async function(){
    if(cfg.on){cfg.on=false;save();paint();toast('Reminders off','🔕');return}
    if(!('Notification' in window)){toast('This browser can’t show notifications. On iPhone, add the app to your Home Screen first.','ℹ️');return}
    var r=Notification.permission;if(r==='default')r=await Notification.requestPermission();
    if(r!=='granted'){toast('Notifications are blocked. Allow them in site settings.','🚫');paint();return}
    cfg.on=true;save();paint();toast('Reminders on','🔔');check();
  };
  $('rm-test').onclick=async function(){
    if(!('Notification' in window)){toast('Notifications aren’t supported here','ℹ️');return}
    var r=Notification.permission;if(r==='default')r=await Notification.requestPermission();
    if(r==='granted'){show('🔔 Test reminder','Notifications are working','test');paint()}else toast('Notifications are blocked','🚫');
  };
  $('rm-brief').onclick=function(){cfg.brief=!cfg.brief;save();paint();if(cfg.brief&&!cfg.on)toast('Turn on Class reminders to receive it','ℹ️')};
  [].forEach.call($('rm-lead').children,function(b){b.onclick=function(){cfg.lead=+b.dataset.l;save();[].forEach.call($('rm-lead').children,function(o){o.classList.toggle('on',o===b)});status('Reminders now fire '+cfg.lead+' min before. Press Sync to update Google Calendar too.')}});
  [].forEach.call($('rm-subj').children,function(b){b.onclick=function(){var k=b.dataset.k;if(cfg.off[k])delete cfg.off[k];else cfg.off[k]=1;save();b.classList.toggle('on',!cfg.off[k])}});
  $('gc-cid').onchange=function(){cfg.cid=this.value.trim();save();if(cfg.cid)loadGIS().catch(function(){})};
  $('gc-sub').onclick=subscribe;$('gc-sync').onclick=sync;$('gc-del').onclick=removeCal;
  if(cfg.cid)loadGIS().catch(function(){});
}
ui();
setInterval(check,30000);
document.addEventListener('visibilitychange',function(){if(!document.hidden)check()});
setTimeout(check,1500);
})();
