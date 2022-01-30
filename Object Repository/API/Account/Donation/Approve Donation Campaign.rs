<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Approve Donation Campaign</name>
   <tag></tag>
   <elementGuidId>6f0c164f-cf0e-41c6-8abf-3c984514d1b7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;comment\&quot;: \&quot;supervisor approve\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${ipAccount}/backoffice/approval-donation-account-agency/${id}/approve</restUrl>
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
      <id>b1c13016-6ce2-43fb-ab90-bc42b708859a</id>
      <masked>false</masked>
      <name>ipAccount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.access_token</defaultValue>
      <description></description>
      <id>93d0b0fc-1744-4b4b-8c29-433ea095eff5</id>
      <masked>false</masked>
      <name>access_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_campaign</defaultValue>
      <description></description>
      <id>caafdc20-128f-4bc3-832d-1faa03a7b7e0</id>
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
