
const utils = require("./utils")

module.exports=[{
  method:'GET',
  path:'/',
},{
  method:'POST',
  path:'/todos',
  headers:{
    'content-type':'application/json',
    'autohorization':'Bearer FAKE_JWT_KEY'
  },
  body:JSON.stringify(utils.todoList),
  onResponse:(status, body, context)=>{
    if (status === 200) {
      const resp = JSON.parse(body)
      context['list_id'] = resp['payload']['id']
    }
  }
},{
  method:'PUT',
  path:'/todos',
  setupRequest:(req, context)=>({
    ...req,
    path:`/todos`,
    headers:{
      'content-type':'application/json',
      'autohorization':'Bearer FAKE_JWT_KEY'
    },
    body:JSON.stringify({
      id: context['list_id'],
      title:"Autocannon title update"
    })
  })
},{
  method: 'POST',
  setupRequest:(req, context)=>({
    ...req,
    path:`/todos/${context['list_id']}/items`,
    headers:{
      'content-type':'application/json',
      'autohorization':'Bearer FAKE_JWT_KEY'
    },
    body:JSON.stringify(utils.todoItem)
  }),
  onResponse:(status, body, context)=>{
    if (status === 200) {
      const resp = JSON.parse(body)
      context['todo_id'] = resp['payload']['id']
    }
  }
},{
  method: 'GET',
  setupRequest:(req, context)=>({
    ...req,
    path:`/todos/${context['list_id']}/items`,
    headers:{
      'content-type':'application/json',
      'autohorization':'Bearer FAKE_JWT_KEY'
    }
  })
// },{
//   method:"DELETE",
//   setupRequest:(req, context)=>{
//     let id=1
//     if (context['todo_id']){
//       id=context['todo_id']
//     }
//     return {
//       ...req,
//       path:`/todo/item/${id}`,
//       headers:{
//         'content-type':'application/json',
//         'autohorization':'Bearer FAKE_JWT_KEY'
//       }
//     }
//   }
}]