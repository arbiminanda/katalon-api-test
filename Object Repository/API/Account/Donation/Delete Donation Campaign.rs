<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete Donation Campaign</name>
   <tag></tag>
   <elementGuidId>f89471d3-11e5-472d-b6aa-30a063e4e41b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${access_token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Platform</name>
      <type>Main</type>
      <value>BACKOFFICE</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${ipAccount}/backoffice/donation-account-agency/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url_account</defaultValue>
      <description></description>
      <id>aa18acaf-67e4-4012-90b1-71cddaa0495d</id>
      <masked>false</masked>
      <name>ipAccount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.access_token</defaultValue>
      <description></description>
      <id>e70d64b9-7689-48f0-b72e-e22dff250add</id>
      <masked>false</masked>
      <name>access_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_campaign</defaultValue>
      <description></description>
      <id>15f7fa3d-188f-4792-bf26-aef9d77c5d15</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
