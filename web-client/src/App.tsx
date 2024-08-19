import { useState } from 'react'
import { MauriceApi } from './api'

export const App = () => {
  const [name, setName] = useState('')
  const [serverResponse, setServerResponse] = useState('')

  const getServerResponse = async () => {
    MauriceApi.postApiSayHello({name})
      .then((response) => setServerResponse(response.message))
  }

  return (
    <div>
      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <button onClick={getServerResponse}>Say Hello</button>
      <div>{serverResponse}</div>
    </div>
  )
}
