<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Donation Campaign</name>
   <tag></tag>
   <elementGuidId>7cc387d4-7864-402a-b1ec-fb9843100ca9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;data&quot;,
      &quot;value&quot;: &quot;{ \&quot;campaignType\&quot;: \&quot;SEDEKAH_SIGNUP\&quot;, \&quot;campaignCode\&quot;: \&quot;${code}\&quot;, \&quot;campaignName\&quot;: \&quot;Create From Automation\&quot;, \&quot;institutionName\&quot;: \&quot;Badan Amil Zakat Nasional\&quot;, \&quot;donationAmount\&quot;: 25000, \&quot;quotaLimit\&quot;: 1000, \&quot;accountNo\&quot;: \&quot;11100220233\&quot;, \&quot;accountName\&quot;: \&quot;Badan Amil Zakat Nasional\&quot;, \&quot;timeValidityStart\&quot;: \&quot;2021-12-29\&quot;, \&quot;timeValidityEnd\&quot;: \&quot;2021-12-31\&quot;, \&quot;language\&quot;: \&quot;ID\&quot;, \&quot;titleId\&quot;: \&quot;Sedekah Jum\u0027at\&quot;, \&quot;titleEn\&quot;: null, \&quot;accountDescriptionId\&quot;: \&quot;Hari Jumat disebut sebagai sayyidul ayyam atau pemimpin hari-hari lainnya.\&quot;, \&quot;accountDescriptionEn\&quot;: null, \&quot;titleCampaignId\&quot;: \&quot;Keutamaan Sedekah Jum\u0027at\&quot;, \&quot;titleCampaignEn\&quot;: null, \&quot;descriptionCampaignId\&quot;: \&quot;Tahukah kamu? Bahwasannya Allah Subhanahu wa Ta’ala lebih mencintai amalan yang dilakukan secara istiqomah walaupun alaman itu sedikit, dari pada amalan yang banyak tapi itu hanya sekali dalam seumur hidup. Bersama Hijra kita wujudkan pola hidup “Tiada hari tanpa sedekah, walau dengan seribu Rupiah”\&quot;, \&quot;descriptionCampaignEn\&quot;: null, \&quot;testimoniName\&quot;: null, \&quot;testimoniTeksId\&quot;: null, \&quot;testimoniTeksEn\&quot;: null }&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;imageInstitution&quot;,
      &quot;value&quot;: &quot;img.PNG&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;imageCampaign&quot;,
      &quot;value&quot;: &quot;img.PNG&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;imageTestimoni&quot;,
      &quot;value&quot;: &quot;img.PNG&quot;,
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
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ipAccount}/backoffice/donation-account-agency</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
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
      <defaultValue>GlobalVariable.donation_campaign_code</defaultValue>
      <description></description>
      <id>4ac5cddd-860c-43ef-9b37-afcb3c24fdd9</id>
      <masked>false</masked>
      <name>code</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
