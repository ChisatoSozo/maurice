import SwaggerUI from "swagger-ui-react"
import "swagger-ui-react/swagger-ui.css"


export const Swagger = () => {
  return (
    <SwaggerUI url="http://192.168.2.56:8080/api/spec/v2.json"/>
  )
}
