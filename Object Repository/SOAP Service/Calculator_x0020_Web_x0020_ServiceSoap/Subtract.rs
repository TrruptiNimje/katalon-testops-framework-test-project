<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Subtract</name>
   <tag></tag>
   <elementGuidId>ae37ce2d-6adb-4c89-b3e2-9f9d201b5405</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/Subtract</value>
      <webElementGuid>edb0a5f4-5181-4004-a1a9-a33f88dd9803</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>649ef3fe-da50-4ee6-9964-d546b2a7abdf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <path></path>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:Subtract>
         &lt;tem:a>${num1}&lt;/tem:a>
         &lt;tem:b>3&lt;/tem:b>
      &lt;/tem:Subtract>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://ecs.syr.edu/faculty/fawcett/Handouts/cse775/code/calcWebService/Calc.asmx</soapServiceEndpoint>
   <soapServiceFunction>Subtract</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'5'</defaultValue>
      <description></description>
      <id>2e948d65-d097-46cf-893c-50e918174c0a</id>
      <masked>false</masked>
      <name>num1</name>
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

assertThat(response.getStatusCode()).isEqualTo(200)
WS.verifyElementText(response, 'SubtractResponse.SubtractResult', '2')</verificationScript>
   <wsdlAddress>https://ecs.syr.edu/faculty/fawcett/Handouts/cse775/code/calcWebService/Calc.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
