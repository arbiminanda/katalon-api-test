<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Donation Campaign</name>
   <tag></tag>
   <elementGuidId>f035e63b-938f-4da1-b177-3c8f157d08d2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;data&quot;,
      &quot;value&quot;: &quot;{ \&quot;campaignName\&quot;: \&quot;Sedekah Jum\u0027at-Update\&quot;, \&quot;institutionName\&quot;: \&quot;Badan Amil Zakat-Nasional Update\&quot;, \&quot;donationAmount\&quot;: 50000, \&quot;quotaLimit\&quot;: 500, \&quot;accountNo\&quot;: \&quot;111002202456\&quot;, \&quot;accountName\&quot;: \&quot;Badan Amil Zakat Nasional-Update\&quot;, \&quot;timeValidityStart\&quot;: \&quot;2021-12-30\&quot;, \&quot;timeValidityEnd\&quot;: \&quot;2021-12-30\&quot;, \&quot;language\&quot;: \&quot;ID\&quot;, \&quot;titleId\&quot;: \&quot;Sedekah Jum\u0027at-Update\&quot;, \&quot;titleEn\&quot;: null, \&quot;accountDescriptionId\&quot;: \&quot;Hari Jumat disebut sebagai sayyidul ayyam atau pemimpin hari-hari lainnya.\&quot;, \&quot;accountDescriptionEn\&quot;: null, \&quot;titleCampaignId\&quot;: \&quot;Keutamaan Sedekah Jum\u0027at\&quot;, \&quot;titleCampaignEn\&quot;: null, \&quot;descriptionCampaignId\&quot;: \&quot;Tahukah kamu? Bahwasannya Allah Subhanahu wa Ta’ala lebih mencintai amalan yang dilakukan secara istiqomah walaupun alaman itu sedikit, dari pada amalan yang banyak tapi itu hanya sekali dalam seumur hidup. Bersama Hijra kita wujudkan pola hidup “Tiada hari tanpa sedekah, walau dengan seribu Rupiah”\&quot;, \&quot;descriptionCampaignEn\&quot;: null, \&quot;testimoniName\&quot;: null, \&quot;testimoniTeksId\&quot;: null, \&quot;testimoniTeksEn\&quot;: null }&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;imageInstitution&quot;,
      &quot;value&quot;: &quot;img_width.PNG&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;imageCampaign&quot;,
      &quot;value&quot;: &quot;img_width.PNG&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;imageTestimoni&quot;,
      &quot;value&quot;: &quot;img_width.PNG&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
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
